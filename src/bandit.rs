use arm::Arm;
use rand::Rng;

/// Setup parameters
#[derive(Clone, Debug)]
pub struct Bandit {
    /// Bandit arms
    pub arms: Vec<Arm>
}

impl Bandit {
    /// Returns a randomly initialized bandit.
    pub fn new<R: Rng>(rng: &mut R, num_arms: usize, bias: f64) -> Bandit {
        assert!(num_arms > 0);
        let mut arms: Vec<Arm> = Vec::with_capacity(num_arms);
        for _ in 0 .. num_arms {
            arms.push(Arm::new(rng, bias))
        }
        Bandit { arms }
    }

    /// Returns a uniformly random sample from a particular arm.
    pub fn sample<R: Rng>(&self, rng: &mut R, arm: usize) -> f64 {
        let arm = &self.arms[arm];
        arm.sample(rng)
    }

    /// Returns the optimal action, which is clear internally but not necessarily externally,
    /// because it may take many samples to get a good estimate of the true distribution.
    /// Therefore, use this function to evaluate results only; do not have agents call this
    /// directly.
    pub fn optimal_arm(&self) -> usize {
        let mut best_arm: usize = 0;
        let mut max_mean: f64 = self.arms[0].mean;
        let num_arms = self.arms.len();
        for i in 1 .. num_arms {
            let x = self.arms[i].mean;
            if x > max_mean {
                max_mean = x;
                best_arm = i;
            }
        }
        best_arm
    }
}


