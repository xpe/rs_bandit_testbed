//! Gradient Bandit
//!
//! H_{t+1}(a) = H_t(a) + α (∂𝔼[R_t] / ∂H_t(a))
//!
//! ∂𝔼[R_t] / ∂H_t(a)
//! = 𝔼[(R_t - B_t) (𝟙_(a=A_t) - π_t(a))]
//! = (R_t - B_t) (𝟙_(a=A_t) - π_t(a))
//!
//! Where:
//! t (time step)
//! a (action)
//! π_t (policy at time step t)
//! H_t (action preference)
//! B_t (the baseline) = R_hat_t
//!
//! Part of derivation:
//! ∂π_t(x) / ∂H_t(a)
//! = ∂π_t(x) (𝟙_(a=x) - ∂π_t(a))

const BASELINE: bool = true;

use action::Action;
use action_preferences::AP;
use bandit::Bandit;
use distribution::sample;
use rand::Rng;
use result::Result;

pub struct Gradient;

fn to_string(vec: &Vec<f64>) -> String {
    let mut s = String::new();
    for v in vec {
        s.push_str(&format!("{:+6.2} ", v));
    }
    s.pop();
    s
}

impl Gradient {
    /// Selects and returns one Action.
    ///
    /// Symbols: (code : textbook)
    /// `arm` : `A_t`
    /// `baseline` : `R_bar_t`
    fn action<R: Rng>(
        rng: &mut R, ap: &mut AP, bandit: &Bandit, alpha: f64, baseline: &mut f64, step: usize
    ) -> Action {
        let policy = ap.policy();
        let arm = sample(rng, &policy);
        ap.n[arm] += 1;
        let reward = bandit.sample(rng, arm);

        if BASELINE {
            // Incremental update from page 31, section 2.3 of Sutton and Barto, 2nd Ed.
            *baseline += (reward - *baseline) / step as f64;
        }

        // Incremental formula from page 39, section 2.8 of Sutton and Barto, 2nd Ed.
        for a in 0 .. ap.h.len() { // Consider all actions
            let factor = if a == arm { policy[arm] - 1.0 } else { policy[arm] };
            ap.h[a] -= alpha * (reward - *baseline) * factor;
        }
        if false {
            println!(
                "step:{:4} arm:{} n:{:4} r:{:6.3} b:{:+6.3} [{}] [{}] ",
                step, arm, ap.n[arm], reward, baseline,
                to_string(&ap.h),
                to_string(&ap.policy()));
        }
        Action { arm, reward }
    }

    /// Do `steps` actions. Returns the final result.
    pub fn actions<R: Rng>(
        rng: &mut R, bandit: &Bandit, alpha: f64, steps: usize
    ) -> Result {
        let ap = &mut AP::new(bandit.arms.len());
        let optimal_arm = bandit.optimal_arm();
        let mut rewards: Vec<f64> = Vec::with_capacity(steps);
        let mut optimals: Vec<f64> = Vec::with_capacity(steps);
        let baseline = &mut 0.0;
        for step in 1 ..= steps {
            let action = Self::action(rng, ap, bandit, alpha, baseline, step);
            rewards.push(action.reward);
            let optimal = if action.arm == optimal_arm { 1.0 } else { 0.0 };
            optimals.push(optimal);
        }
        Result { rewards, optimals }
    }
}

