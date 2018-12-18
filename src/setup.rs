/// Setup parameters
#[derive(Clone, Debug)]
pub struct Setup {
    /// Alpha for gradient bandit
    pub alpha: Option<f64>,

    /// Number of bandit arms
    pub arms: usize,

    /// c parameter
    pub c: Option<f64>,

    /// Greediness parameter
    pub epsilon: Option<f64>,

    /// Number of problems
    pub problems: usize,

    /// Initial q value
    pub q0: Option<f64>,

    /// Number of time steps
    pub steps: usize,
}
