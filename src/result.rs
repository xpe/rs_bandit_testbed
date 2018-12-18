use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

/// Result of one or many bandit runs.
#[derive(Clone, Debug)]
pub struct Result {
    /// Cumulative rewards (one for each time step)
    pub rewards: Vec<f64>,

    /// Optimal action fraction (at each time step)
    pub optimals: Vec<f64>,
}

impl Result {
    /// Returns an empty `Result`
    pub fn new(steps: usize) -> Result {
        let rewards = vec![0.0; steps];
        let optimals = vec![0.0; steps];
        Result { rewards, optimals }
    }

    /// Averages the results in place.
    pub fn average(&mut self, other: &Result, n: usize) {
        assert_eq!(self.rewards.len(), self.optimals.len());
        assert_eq!(self.rewards.len(), other.rewards.len());
        assert_eq!(self.optimals.len(), other.optimals.len());
        for i in 0 .. self.rewards.len() {
            self.rewards[i] += (other.rewards[i] - self.rewards[i]) / n as f64;
            self.optimals[i] += (other.optimals[i] - self.optimals[i]) / n as f64;
        }
    }

    /// Writes Result to a CSV file.
    pub fn write_to_csv<P: AsRef<Path>>(&self, path: P) {
        assert_eq!(self.rewards.len(), self.optimals.len());
        let file = File::create(path).expect("Cannot create file");
        let mut writer = BufWriter::new(file);
        let header = "reward,optimal\n";
        writer.write_all(header.as_bytes()).expect("Cannot write header");
        for i in 0 .. self.rewards.len() {
            let line = format!("{},{}\n", self.rewards[i], self.optimals[i]);
            writer.write_all(line.as_bytes()).expect("Cannot write line");
        }
    }
}