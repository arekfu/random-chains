use rand::{thread_rng, Rng};

use super::config::Config;
use super::power_law;
use super::score;

#[derive(Debug)]
pub struct Sample(pub Vec<f64>);

#[derive(Debug)]
pub struct Samples(pub Vec<Sample>);

pub fn run(config: &Config) {
    let mut rng = thread_rng();
    let samples = make_samples(
        config.max_size,
        config.replicas,
        config.budget,
        config.exponent_min,
        config.exponent_max,
        &mut rng,
    );
    let sample_sizes = Samples(
        samples
            .0
            .iter()
            .map(|sample| Sample(vec![sample.0.len() as f64]))
            .collect(),
    );
    let sample_sizes_mean = score::Score::from(&sample_sizes);
    println!("sample sizes: {}", sample_sizes_mean);

    let result = score::Score::from(&samples);
    let theoretical = expected(config.exponent_min, config.exponent_max);
    println!("\ntheoretical: {}", theoretical);
    println!("scored: {}", result);
    println!(
        "discrepancy: {} σ",
        (result.mean - theoretical) / result.std_err
    );
}

fn expected(exp_min: f64, exp_max: f64) -> f64 {
    if exp_max == exp_min {
        (exp_max + 1.0) / (exp_max + 2.0)
    } else {
        1.0 - ((exp_max + 2.0) / (exp_min + 2.0)).ln() / (exp_max - exp_min)
    }
}

fn make_samples<R>(
    max_size: usize,
    replicas: usize,
    max_total: f64,
    exp_min: f64,
    exp_max: f64,
    mut rng: R,
) -> Samples
where
    R: Rng,
{
    Samples(
        (0..replicas)
            .map(|_| sample(max_size, max_total, exp_min, exp_max, &mut rng))
            .collect(),
    )
}

fn sample<R>(max_size: usize, max_total: f64, exp_min: f64, exp_max: f64, rng: R) -> Sample
where
    R: Rng,
{
    let mut total = 0.0;
    let sample_vec: Vec<f64> = rng
        .sample_iter(power_law::VariablePowerLaw::new(exp_min, exp_max))
        .take(max_size)
        .take_while(|x| {
            let res = total < max_total;
            total += x;
            res
        })
        .collect();
    Sample(sample_vec)
}

#[cfg(test)]
mod tests {
    use std::f64;

    const N_REPLICAS: usize = 10000;

    #[test]
    fn check_max_size() {
        let mut rng = super::thread_rng();
        for _ in 0..N_REPLICAS {
            let smpl = super::sample(10, f64::INFINITY, -3.0, -2.0, &mut rng);
            assert_eq!(smpl.0.len(), 10, "sample = {:?}", smpl.0);
        }
    }

    #[test]
    fn check_max_1() {
        let mut rng = super::thread_rng();
        for _ in 0..N_REPLICAS {
            let smpl = super::sample(10, 0.5, -3.0, -2.0, &mut rng);
            assert_eq!(smpl.0.len(), 1);
        }
    }

    #[test]
    fn check_max_4() {
        let mut rng = super::thread_rng();
        for _ in 0..N_REPLICAS {
            let smpl = super::sample(10, 3.5, -3.0, -2.0, &mut rng);
            assert!(smpl.0.len() <= 4);
        }
    }
}