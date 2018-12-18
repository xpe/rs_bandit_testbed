/// AP = Action Preferences
/// An agent's action preferences (internal state)
#[derive(Clone, Debug)]
pub struct AP {
    /// Action preference for each bandit arm
    pub h: Vec<f64>,

    /// Number of trials
    pub n: Vec<usize>,
}

impl AP {
    /// Initialize action preferences.
    pub fn new(num_arms: usize) -> AP {
        AP {
            h: vec![0.0; num_arms],
            n: vec![0; num_arms],
        }
    }

    /// Returns the probabilities for each action. Uses a soft-max transformation.
    pub fn policy(&self) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::with_capacity(self.h.len());
        let mut d = 0.0; // denominator
        for v in &self.h {
            let exp = v.exp();
            d += exp;
            vec.push(exp);
        }
        for i in 0 .. self.h.len() { vec[i] /= d; }
        vec
    }

    /// Return a string representation of the agent's action preferences.
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str("[");
        for i in 0 .. self.h.len() {
            s.push_str(format!("{:+0.2} ", self.h[i]).as_str())
        }
        s.pop();
        s.push_str("]");
        s
    }
}
