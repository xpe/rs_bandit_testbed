/// Action (both the arm chosen and the ensuing reward)
#[derive(Clone, Debug)]
pub struct Action {
    /// The chosen arm
    pub arm: usize,

    /// The reward
    pub reward: f64,
}
