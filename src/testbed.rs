extern crate rand;

use agents::gradient::Gradient;
use agents::greedy::Greedy;
use agents::ucb::UCB;
use bandit::Bandit;
use rand::thread_rng;
use result::Result;
use setup::Setup;
use std::process::exit;

pub fn run(setup: Setup) -> Result {
    let rng = &mut thread_rng();
    let steps = setup.steps;
    let mut result = Result::new(steps);
    let q0 = setup.q0.unwrap_or(0.0);
    for n in 1 ..= setup.problems {
        if let Some(alpha) = setup.alpha {
            let bandit = Bandit::new(rng, setup.arms, 4.0);
            let final_result = Gradient::actions(rng, &bandit, alpha, steps);
            result.average(&final_result, n);
        } else if let Some(epsilon) = setup.epsilon {
            let bandit = Bandit::new(rng, setup.arms, 0.0);
            let final_result = Greedy::actions(rng, &bandit, epsilon, q0, steps);
            result.average(&final_result, n);
        } else if let Some(c) = setup.c {
            let bandit = Bandit::new(rng, setup.arms, 0.0);
            let final_result = UCB::actions(rng, &bandit, c, q0, steps);
            result.average(&final_result, n);
        } else {
            println!("Incorrect arguments.");
            exit(1);
        };
    }
    result
}

