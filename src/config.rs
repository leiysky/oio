use std::{fmt::Display, time::Duration};

use error_stack::{bail, Report, Result};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ConfigError(pub String);

/// Configuration of oio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Object storage service configuration
    pub service: Service,
    pub job: JobConfig,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.job.file_size < 4096 {
            bail!(ConfigError(
                "file_size must be greater or equal to 4096".to_string()
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    /// Endpoint of object storage, e.g. s3.us-east-1.amazonaws.com
    pub endpoint: String,
    /// Service name, e.g. s3, oss, minio
    #[serde(rename = "type")]
    pub type_: ServiceType,
    /// Bucket name, e.g. my-bucket
    pub bucket: String,
    /// Prefix of object, e.g. path/to/
    /// Default: ""
    pub prefix: Option<String>,
    /// Region
    pub region: Option<String>,
    /// Access key
    pub access_key: String,
    /// Secret key
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    /// Number of jobs
    /// Default: 1
    pub num_jobs: Option<u32>,
    /// Workload for testing
    pub workload: Workload,
    /// Size of file in bytes
    pub file_size: u64,
    /// Maximum time to run the job
    #[serde(with = "humantime_serde")]
    pub run_time: Duration,
}

/// Service kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceType {
    /// AWS S3
    S3,
    /// AliCloud OSS
    Oss,
    /// Minio object storage
    Minio,

    /// Local file system
    Fs,
}

impl Serialize for ServiceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'a> Deserialize<'a> for ServiceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        ServiceType::try_from(s.as_str()).map_err(serde::de::Error::custom)
    }
}

impl TryFrom<&str> for ServiceType {
    type Error = Report<ConfigError>;

    fn try_from(value: &str) -> Result<Self, ConfigError> {
        match value {
            "s3" => Ok(ServiceType::S3),
            "oss" => Ok(ServiceType::Oss),
            "minio" => Ok(ServiceType::Minio),
            "fs" => Ok(ServiceType::Fs),
            _ => bail!(ConfigError(format!("invalid service: {}", value))),
        }
    }
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceType::S3 => write!(f, "s3"),
            ServiceType::Oss => write!(f, "oss"),
            ServiceType::Minio => write!(f, "minio"),
            ServiceType::Fs => write!(f, "fs"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Workload {
    Download,
    Upload,
}

impl Display for Workload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Workload::Download => write!(f, "download"),
            Workload::Upload => write!(f, "upload"),
        }
    }
}

impl TryFrom<&str> for Workload {
    type Error = Report<ConfigError>;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "download" => Ok(Workload::Download),
            "upload" => Ok(Workload::Upload),
            _ => bail!(ConfigError(format!("invalid workload: {}", value))),
        }
    }
}

impl Serialize for Workload {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'a> Deserialize<'a> for Workload {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        Workload::try_from(s.as_str()).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn test_parse_config() {
        let config = r#"
        [service]
        endpoint = "aws.us-east-1.amazonaws.com"
        type = "s3"
        bucket = "my-bucket"
        prefix = "path/to/"
        access_key = "AKIAIOSFODNN7EXAMPLE"
        secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"

        [job]
        num_jobs = 4
        run_time = "1min"
        file_size = 1024
        workload = "download"
        "#;

        let config: Config = toml::from_str(config).unwrap();
        assert_snapshot!(toml::to_string_pretty(&config).unwrap(), @r###"
        [service]
        endpoint = "aws.us-east-1.amazonaws.com"
        type = "s3"
        bucket = "my-bucket"
        prefix = "path/to/"
        access_key = "AKIAIOSFODNN7EXAMPLE"
        secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"

        [job]
        num_jobs = 4
        workload = "download"
        file_size = 1024
        run_time = "1m"
        "###);
    }
}
