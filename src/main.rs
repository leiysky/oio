mod config;
mod job;
mod report;
mod sample;

use config::Config;
use error_stack::{Result, ResultExt};
use job::Job;
use report::Report;
use std::{fs::File, io::Read, process::exit};

use clap::Parser;
use thiserror::Error;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    config_file: String,
}

fn main() {
    let args = Args::parse();

    match run(&args) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{:?}", e);
            exit(1);
        }
    }
}

#[derive(Debug, Error)]
#[error("{0}")]
struct CliError(pub String);

fn run(args: &Args) -> Result<(), CliError> {
    let error = || CliError("failed to run job".to_string());

    let mut config_file = File::open(&args.config_file).change_context_lazy(error)?;
    let mut buf = vec![];
    config_file
        .read_to_end(&mut buf)
        .change_context_lazy(error)?;
    let config_str = String::from_utf8(buf).change_context_lazy(error)?;
    let config: Config = toml::from_str(&config_str).change_context_lazy(error)?;
    config.validate().change_context_lazy(error)?;

    let mut job = Job::new(config.clone());
    let (bandwidth, latency, iops) = job.run().change_context_lazy(error)?;

    let report = Report::new(
        config.job.num_jobs.unwrap_or(1),
        config.job.file_size,
        config.job.workload.to_string(),
        bandwidth,
        latency,
        iops,
    );
    println!("{}", report);

    Ok(())
}
