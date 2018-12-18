/// AVE = Action Value Estimates
/// An agent's action value estimates (internal state)
#[derive(Clone, Debug)]
pub struct AVE {
    /// Action value estimate for each bandit arm
    pub q: Vec<f64>,

    /// Number of trials
    pub n: Vec<usize>,
}

impl AVE {
    /// Initialize action value estimates.
    pub fn new(num_arms: usize, q0: f64) -> AVE {
        AVE {
            q: vec![q0; num_arms],
            n: vec![0; num_arms],
        }
    }

    /// Return a string representation of the agent's action value estimates.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("[");
        for i in 0 .. self.q.len() {
            s.push_str(format!("{:+0.2} ", self.q[i]).as_str())
        }
        s.pop();
        s.push_str("]");
        s
    }
}
