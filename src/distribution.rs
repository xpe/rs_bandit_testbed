use rand::Rng;

/// Returns cdf (cumulative distribution function) based on a pdf (probability density function).
pub fn cumulative(policy: &Vec<f64>) -> Vec<f64> {
    let mut sum = 0.0;
    let mut vec= Vec::with_capacity(policy.len());
    for v in policy {
        sum += v;
        vec.push(sum); }
    vec
}

/// Sample from a probability distribution (i.e. a policy).
pub fn sample<R: Rng>(rng: &mut R, policy: &Vec<f64>) -> usize {
    let x = rng.gen_range(0.0, 1.0);
    let cdf = cumulative(policy);
    let mut choice: Option<usize> = None;
    for i in 0 .. cdf.len() {
        if x <= cdf[i] { choice = Some(i); break; }
    }
    match choice {
        Some(v) => v,
        None =>  {
            println!("x : {}", x);
            println!("policy : {:?}", policy);
            println!("cdf : {:?}", cdf);
            panic!("Sampling from a malformed distribution not supported.");
        },
    }
}

