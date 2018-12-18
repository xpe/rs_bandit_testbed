extern crate bandit;
extern crate clap;

use bandit::setup::Setup;
use bandit::testbed;
use clap::{App, Arg, ArgMatches};
use std::fs;

fn parse_usize(matches: &ArgMatches, s: &str) -> usize {
    matches.value_of(s).unwrap().parse().unwrap()
}

fn parse_opt_f64(matches: &ArgMatches, s: &str) -> Option<f64> {
    matches.value_of(s).map(|x| x.parse().unwrap())
}

fn cli<'a>() -> ArgMatches<'a> {
    App::new("Multi-Armed Bandit")
        .version("0.1")
        .author("David James <davidcjames@gmail.com>")
        .about("Multi-Armed Bandit from Sutton and Barto")
        .arg(Arg::with_name("alpha")
            .short("a")
            .long("alpha")
            .takes_value(true)
            .help("Alpha parameter for gradient bandit")
            .required(false))
        .arg(Arg::with_name("c")
            .help("UCB parameter")
            .short("c")
            .long("c")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("arms")
            .help("Number of bandit arms")
            .required(true))
        .arg(Arg::with_name("epsilon")
            .short("e")
            .long("epsilon")
            .takes_value(true)
            .help("Greediness parameter")
            .required(false))
        .arg(Arg::with_name("problems")
            .help("Number of problems")
            .required(true))
        .arg(Arg::with_name("q0")
            .long("q0")
            .takes_value(true)
            .help("Initial q")
            .required(false))
        .arg(Arg::with_name("steps")
            .help("Number of time steps")
            .required(true))
        .arg(Arg::with_name("v")
            .help("Verbosity level")
            .short("v")
            .multiple(true))
        .get_matches()
}

fn main() {
    let matches = cli();

    let alpha = parse_opt_f64(&matches, "alpha");
    let arms = parse_usize(&matches, "arms");
    let c = parse_opt_f64(&matches, "c");
    let epsilon = parse_opt_f64(&matches, "epsilon");
    let problems = parse_usize(&matches, "problems");
    let q0 = parse_opt_f64(&matches, "q0");
    let steps = parse_usize(&matches, "steps");

    let gradient = alpha.is_some();
    let ucb = c.is_some();

    let setup = Setup { alpha, arms, c, epsilon, problems, q0, steps };
    println!("Running {:?}", setup);
    let result = testbed::run(setup);
    let subdir = format!("a={}_p={}_s={}", arms, problems, steps);
    let dir = format!("results/{}", subdir);
    fs::create_dir_all(&dir).expect("Cannot create directory");
    let filename = if gradient {
        format!("{}/alpha={}.csv", dir, alpha.unwrap())
    } else if ucb {
        format!("{}/c={}.csv", dir, c.unwrap())
    } else {
        format!("{}/eps={}.csv", dir, epsilon.unwrap())
    };
    println!("Writing results to {}", filename);
    result.write_to_csv(filename);
}
