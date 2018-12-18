use action::Action;
use action_value_estimates::AVE;
use bandit::Bandit;
use rand::Rng;
use result::Result;

pub struct UCB;

impl UCB {
    /// Returns the bandit arm with the highest action value estimate. Breaks ties randomly.
    fn ucb<R: Rng>(rng: &mut R, ave: &AVE, c: f64, step: usize) -> usize {
        let num_arms = ave.q.len();
        let initial_arm: usize = rng.gen_range(0, ave.q.len());
        let mut best_arm = initial_arm;
        let mut max_val = ave.q[best_arm];
        let t = step as f64;
        for i in initial_arm .. initial_arm + num_arms {
            let arm = i % num_arms;
            let val = ave.q[arm] + c * (t.ln() / (ave.n[arm] as f64)).sqrt();
            if val > max_val {
                max_val = val;
                best_arm = arm;
            }
        }
        best_arm
    }

    /// Returns one Action. Mutates the action value estimates argument.
    fn action<R: Rng>(
        rng: &mut R, ave: &mut AVE, bandit: &Bandit, c: f64, step: usize
    ) -> Action {
        let arm = Self::ucb(rng, ave, c, step);
        let reward = bandit.sample(rng, arm);
        // Incremental formula from section 2.4 of Sutton and Barto, 2nd Ed.
        ave.n[arm] += 1;
        ave.q[arm] += (reward - ave.q[arm]) / ave.n[arm] as f64;
        Action { arm, reward }
    }

    /// Do `steps` actions. Returns the final result.
    pub fn actions<R: Rng>(
        rng: &mut R, bandit: &Bandit, c: f64, q0: f64, steps: usize
    ) -> Result {
        let ave = &mut AVE::new(bandit.arms.len(), q0);
        let optimal_arm = bandit.optimal_arm();
        let mut rewards: Vec<f64> = Vec::with_capacity(steps);
        let mut optimals: Vec<f64> = Vec::with_capacity(steps);
        for step in 1 ..= steps {
            let action = Self::action(rng, ave, bandit, c, step);
            rewards.push(action.reward);
            let optimal = if action.arm == optimal_arm { 1.0 } else { 0.0 };
            optimals.push(optimal);
        }
        Result { rewards, optimals }
    }
}

