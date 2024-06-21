use std::{fmt::Display, time::Duration};

use serde::{Deserialize, Serialize};

use crate::sample::SampleSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    num_samples: u32,
    min: f64,
    max: f64,
    avg: f64,
    stdev: f64,
    p99: f64,
    p95: f64,
    p50: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// throughput in bytes/s
    bandwidth: Metric,
    /// latency in microseconds
    latency: Metric,
}

impl Report {
    pub fn new(bandwidth: SampleSet, latency: SampleSet) -> Self {
        Self {
            bandwidth: Metric {
                num_samples: bandwidth.num_samples() as u32,
                min: bandwidth.min(),
                max: bandwidth.max(),
                avg: bandwidth.avg(),
                stdev: bandwidth.stdev(),
                p99: bandwidth.percentile(99.0),
                p95: bandwidth.percentile(95.0),
                p50: bandwidth.percentile(50.0),
            },
            latency: Metric {
                num_samples: latency.num_samples() as u32,
                min: latency.min(),
                max: latency.max(),
                avg: latency.avg(),
                stdev: latency.stdev(),
                p99: latency.percentile(99.0),
                p95: latency.percentile(95.0),
                p50: latency.percentile(50.0),
            },
        }
    }
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bandwidth:")?;
        writeln!(f, "  num_samples: {}", self.bandwidth.num_samples)?;
        writeln!(
            f,
            "  min: {}/s",
            humansize::format_size(self.bandwidth.min as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  max: {}/s",
            humansize::format_size(self.bandwidth.max as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  avg: {}/s",
            humansize::format_size(self.bandwidth.avg as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  stdev: {}/s",
            humansize::format_size(self.bandwidth.stdev as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  p99: {}/s",
            humansize::format_size(self.bandwidth.p99 as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  p95: {}/s",
            humansize::format_size(self.bandwidth.p95 as u64, humansize::BINARY)
        )?;
        writeln!(
            f,
            "  p50: {}/s",
            humansize::format_size(self.bandwidth.p50 as u64, humansize::BINARY)
        )?;
        writeln!(f)?;
        writeln!(f, "Latency:")?;
        writeln!(f, "  num_samples: {}", self.latency.num_samples)?;
        writeln!(
            f,
            "  min: {}",
            humantime::format_duration(Duration::from_micros(self.latency.min as u64))
        )?;
        writeln!(
            f,
            "  max: {}",
            humantime::format_duration(Duration::from_micros(self.latency.max as u64))
        )?;
        writeln!(
            f,
            "  avg: {}",
            humantime::format_duration(Duration::from_micros(self.latency.avg as u64))
        )?;
        writeln!(
            f,
            "  stdev: {}",
            humantime::format_duration(Duration::from_micros(self.latency.stdev as u64))
        )?;
        writeln!(
            f,
            "  p99: {}",
            humantime::format_duration(Duration::from_micros(self.latency.p99 as u64))
        )?;
        writeln!(
            f,
            "  p95: {}",
            humantime::format_duration(Duration::from_micros(self.latency.p95 as u64))
        )?;
        writeln!(
            f,
            "  p50: {}",
            humantime::format_duration(Duration::from_micros(self.latency.p50 as u64))
        )?;
        Ok(())
    }
}
