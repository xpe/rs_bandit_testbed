use rand::Rng;
use rand::distributions::{Normal, StandardNormal};

/// Setup parameters
#[derive(Clone, Debug)]
pub struct Arm {
    /// Mean
    pub mean: f64,

    /// Standard Deviation
    pub std_dev: f64,
}

impl Arm {
    /// Returns a randomly initialized arm.
    pub fn new<R: Rng>(rng: &mut R, bias: f64) -> Arm {
        Arm {
            mean: bias + rng.sample(StandardNormal),
            std_dev: 1.0,
        }
    }

    /// Returns a sample from the arm.
    pub fn sample<R: Rng>(&self, rng: &mut R) -> f64 {
        let normal = Normal::new(self.mean, self.std_dev);
        rng.sample(normal)
    }
}
