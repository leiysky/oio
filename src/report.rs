use std::fmt::Display;

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
    /// throughput in KiB/s
    bandwidth: Metric,
    /// latency in ms
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
        writeln!(f, "  min(KiB/s): {:.3}", self.bandwidth.min)?;
        writeln!(f, "  max(KiB/s): {:.3}", self.bandwidth.max)?;
        writeln!(f, "  avg(KiB/s): {:.3}", self.bandwidth.avg)?;
        writeln!(f, "  stdev(KiB/s): {:.3}", self.bandwidth.stdev)?;
        writeln!(f, "  p99(KiB/s): {:.3}", self.bandwidth.p99)?;
        writeln!(f, "  p95(KiB/s): {:.3}", self.bandwidth.p95)?;
        writeln!(f, "  p50(KiB/s): {:.3}", self.bandwidth.p50)?;
        writeln!(f)?;
        writeln!(f, "Latency:")?;
        writeln!(f, "  num_samples: {}", self.latency.num_samples)?;
        writeln!(f, "  min(ms): {:.3}", self.latency.min)?;
        writeln!(f, "  max(ms): {:.3}", self.latency.max)?;
        writeln!(f, "  avg(ms): {:.3}", self.latency.avg)?;
        writeln!(f, "  stdev(ms): {:.3}", self.latency.stdev)?;
        writeln!(f, "  p99(ms): {:.3}", self.latency.p99)?;
        writeln!(f, "  p95(ms): {:.3}", self.latency.p95)?;
        writeln!(f, "  p50(ms): {:.3}", self.latency.p50)?;
        Ok(())
    }
}
