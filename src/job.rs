use crate::{
    config::{Config, Service, ServiceType, Workload},
    sample::SampleSet,
};
use bytes::{BufMut, Bytes, BytesMut};
use error_stack::{Result, ResultExt};
use opendal::Operator;
use thiserror::Error;
use tokio::task::JoinHandle;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct JobError(pub String);

pub struct Job {
    config: Config,
}

impl Job {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Run job, return sample set of (Bandwidth, Latency)
    pub fn run(&mut self) -> Result<(SampleSet, SampleSet), JobError> {
        let error = || JobError("failed to run job".to_string());
        let num_jobs = self.config.job.num_jobs.unwrap_or(1);
        let start = std::time::Instant::now();
        let run_time = self.config.job.run_time;
        let operator = build_operator(&self.config.service)?;

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(num_jobs as usize)
            .enable_all()
            .build()
            .change_context_lazy(error)?;

        let task = runtime.block_on(async { self.prepare_task().await })?;

        let mut handles: Vec<JoinHandle<Result<_, JobError>>> = vec![];

        for _ in 0..num_jobs {
            let operator = operator.clone();
            let task = task.clone();
            handles.push(runtime.spawn(async move {
                let mut bandwidth = SampleSet::default();
                let mut latency = SampleSet::default();
                loop {
                    if start.elapsed() > run_time {
                        return Ok((bandwidth, latency));
                    }
                    let task_start = std::time::Instant::now();
                    let bytes = task.run(&operator).await?;
                    let lat = task_start.elapsed();
                    latency.add(lat.as_micros() as f64);
                    bandwidth.add(bytes as f64 / lat.as_secs_f64());
                }
            }));
        }

        let mut bandwidth = SampleSet::default();
        let mut latency = SampleSet::default();
        for handle in handles {
            let (bw, lat) =
                runtime.block_on(async { handle.await.change_context_lazy(error) })??;
            bandwidth = bandwidth.merge(bw);
            latency = latency.merge(lat);
        }

        Ok((bandwidth, latency))
    }

    async fn prepare_task(&self) -> Result<Task, JobError> {
        let error = || JobError("failed to prepare task".to_string());

        let path = format!("oio-test-{}", uuid::Uuid::new_v4());
        let mut content = BytesMut::with_capacity(self.config.job.file_size as usize);
        for _ in 0..self.config.job.file_size {
            content.put_u64(rand::random());
        }
        match self.config.job.workload {
            Workload::Download => {
                let operator = build_operator(&self.config.service)?;

                operator
                    .write_with(&path, content.freeze())
                    .await
                    .change_context_lazy(error)?;
                Ok(Task::Download { path })
            }
            Workload::Upload => Ok(Task::Upload {
                path,
                content: content.freeze(),
            }),
        }
    }
}

#[derive(Clone, Debug)]
enum Task {
    Download { path: String },
    Upload { path: String, content: Bytes },
}

impl Task {
    /// Run task with operator, returns processed bytes
    pub async fn run(&self, operator: &Operator) -> Result<u64, JobError> {
        match self {
            Task::Download { path } => {
                let res = operator.read_with(path).await.change_context_lazy(|| {
                    JobError(format!("failed to download object: {}", path))
                })?;
                Ok(res.len() as u64)
            }
            Task::Upload { path, content } => {
                operator
                    .write_with(path, content.clone())
                    .await
                    .change_context_lazy(|| {
                        JobError(format!("failed to upload object: {}", path))
                    })?;
                Ok(content.len() as u64)
            }
        }
    }
}

/// Build OpenDAL operator from config
fn build_operator(service: &Service) -> Result<Operator, JobError> {
    let operator = match service.type_ {
        ServiceType::S3 | ServiceType::Minio => {
            let mut builder = opendal::services::S3::default();
            if let Some(prefix) = &service.prefix {
                builder.root(prefix);
            }
            builder
                .endpoint(&service.endpoint)
                .bucket(&service.bucket)
                .access_key_id(&service.access_key)
                .secret_access_key(&service.secret_key);
            Operator::new(builder)
                .change_context_lazy(|| JobError("failed to build s3 operator".to_string()))?
                .finish()
        }
        ServiceType::Oss => {
            let mut builder = opendal::services::Oss::default();
            if let Some(prefix) = &service.prefix {
                builder.root(prefix);
            }
            builder
                .endpoint(&service.endpoint)
                .bucket(&service.bucket)
                .access_key_id(&service.access_key)
                .access_key_secret(&service.secret_key);
            Operator::new(builder)
                .change_context_lazy(|| JobError("failed to build oss operator".to_string()))?
                .finish()
        }
        ServiceType::Fs => {
            let mut builder = opendal::services::Fs::default();
            if let Some(prefix) = &service.prefix {
                builder.root(prefix);
            } else {
                builder.root("/tmp/oio");
            }
            Operator::new(builder)
                .change_context_lazy(|| JobError("failed to build fs operator".to_string()))?
                .finish()
        }
    };

    Ok(operator)
}
