#[derive(Debug, Clone, Default)]
pub struct SampleSet(pub Vec<f64>);

impl SampleSet {
    /// Add a new sample
    pub fn add(&mut self, sample: f64) {
        self.0.push(sample);
    }

    /// Merge two sample set
    pub fn merge(mut self, other: SampleSet) -> Self {
        self.0.extend(other.0);
        self
    }

    /// Get number of samples
    pub fn num_samples(&self) -> usize {
        self.0.len()
    }

    /// Get min value
    pub fn min(&self) -> f64 {
        self.0.iter().copied().fold(f64::INFINITY, |a, b| a.min(b))
    }

    /// Get max value
    pub fn max(&self) -> f64 {
        self.0
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, |a, b| a.max(b))
    }

    /// Get average value
    pub fn avg(&self) -> f64 {
        self.0.iter().copied().sum::<f64>() / self.0.len() as f64
    }

    /// Get standard deviation
    pub fn stdev(&self) -> f64 {
        let avg = self.avg();
        let sum = self
            .0
            .iter()
            .copied()
            .map(|x| (x - avg).powi(2))
            .sum::<f64>();
        (sum / self.0.len() as f64).sqrt()
    }

    /// Get percentile value
    pub fn percentile(&self, percentile: f64) -> f64 {
        let mut sorted = self.0.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sorted
            .get(((sorted.len() - 1) as f64 * percentile / 100.0) as usize)
            .copied()
            .unwrap_or(f64::NAN)
    }
}
