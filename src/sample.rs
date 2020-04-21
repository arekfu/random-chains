use rand::{thread_rng, Rng};
use std::iter::Iterator;
use std::time;

use super::config::Config;
use super::power_law;
use super::score;

#[derive(Debug)]
pub struct Sample(pub Vec<f64>);

#[derive(Debug)]
pub struct Samples(pub Vec<Sample>);

pub fn run(config: &Config) {
    let mut rng = thread_rng();
    let now = time::Instant::now();
    let samples = if config.cutoff <= 0.0 {
        make_samples(
            &config,
            &|param| power_law::PowerLaw::with_value(param),
            &mut rng,
        )
    } else {
        make_samples(
            &config,
            &|param| power_law::PowerLawCutoff::with_value(param, config.cutoff),
            &mut rng,
        )
    };
    let sample_sizes = samples
        .0
        .iter()
        .map(|sample| score::Observable::new(sample.0.len() as f64));
    let sample_sizes_mean = score::Score::score(sample_sizes, &config.score_method);
    println!("sample sizes: {}", sample_sizes_mean);

    let sample_tots = samples.0.iter().map(score::Observable::tot);
    let sample_tots_mean = score::Score::score(sample_tots, &config.score_method);
    println!("sample tot: {}", sample_tots_mean);
    println!(
        "sample tot / sample sizes: {}",
        sample_tots_mean.mean / sample_sizes_mean.mean
    );

    let values = samples.0.iter().map(score::Observable::score);
    let result = score::Score::score(values, &config.score_method);

    let theoretical = expected(config.exponent_min, config.exponent_max, config.cutoff);

    println!("\ntheoretical: {}", theoretical);
    println!("scored: {}", result);
    println!(
        "discrepancy: {} σ",
        (result.mean - theoretical) / result.std_err
    );
    let elapsed = now.elapsed().as_millis();
    let elapsed_s = 1e-3 * elapsed as f64;
    println!(
        "elapsed: {} s ({} us / replica, {} us / sample)",
        elapsed_s,
        1000.0 * elapsed as f64 / config.replicas as f64,
        1000.0 * elapsed as f64 / (config.replicas as f64 * sample_sizes_mean.mean)
    );
}

fn expected_cutoff(exp: f64, _: &mut ()) -> f64 {
    rgsl::gamma_beta::incomplete_gamma::gamma_inc(2.0 + exp, 1.0)
        / rgsl::gamma_beta::incomplete_gamma::gamma_inc(1.0 + exp, 1.0)
}

#[allow(clippy::float_cmp)]
fn expected(exp_min: f64, exp_max: f64, cutoff: f64) -> f64 {
    if cutoff <= 0.0 {
        if exp_max == exp_min {
            (exp_max + 1.0) / (exp_max + 2.0)
        } else {
            1.0 - ((exp_max + 2.0) / (exp_min + 2.0)).ln() / (exp_max - exp_min)
        }
    } else if exp_max == exp_min {
        expected_cutoff(exp_max, &mut ())
    } else {
        let mut result: f64 = 0.0;
        let mut abs_err: f64 = 0.0;
        let mut n_eval: usize = 0;
        rgsl::integration::qng(
            expected_cutoff,
            &mut (),
            exp_min,
            exp_max,
            1e-5,
            0.0,
            &mut result,
            &mut abs_err,
            &mut n_eval,
        );
        result
    }
}

fn make_samples<R, F, U>(config: &Config, dependent_distr: &F, mut rng: R) -> Samples
where
    R: Rng,
    F: Fn(f64) -> U,
    U: rand_distr::Distribution<f64>,
{
    Samples(
        (0..config.replicas)
            .map(|_| {
                sample(
                    config.min_size,
                    config.max_size,
                    config.budget,
                    config.exponent_min,
                    config.exponent_max,
                    &dependent_distr,
                    &mut rng,
                )
            })
            .collect(),
    )
}

fn sample<R, F, U>(
    min_size: usize,
    max_size: usize,
    max_total: f64,
    exp_min: f64,
    exp_max: f64,
    dependent_distr: &F,
    rng: R,
) -> Sample
where
    R: Rng,
    F: Fn(f64) -> U,
    U: rand_distr::Distribution<f64>,
{
    let mut total = 0.0;
    let mut size: usize = 0;
    let distr = power_law::Compose::with_distrs(
        rand_distr::uniform::Uniform::new_inclusive(exp_min, exp_max),
        dependent_distr,
    );
    let sample_vec: Vec<f64> = rng
        .sample_iter(distr)
        .take_while(|x| {
            let res = total < max_total || size < min_size;
            total += x;
            size += 1;
            res
        })
        .take(max_size)
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
            let smpl = super::sample(
                0,
                10,
                f64::INFINITY,
                -3.0,
                -2.0,
                &|param| super::power_law::PowerLaw::with_value(param),
                &mut rng,
            );
            assert_eq!(smpl.0.len(), 10, "sample = {:?}", smpl.0);
        }
    }

    #[test]
    fn check_max_1() {
        let mut rng = super::thread_rng();
        for _ in 0..N_REPLICAS {
            let smpl = super::sample(
                0,
                10,
                0.5,
                -3.0,
                -2.0,
                &|param| super::power_law::PowerLaw::with_value(param),
                &mut rng,
            );
            assert_eq!(smpl.0.len(), 1);
        }
    }

    #[test]
    fn check_max_4() {
        let mut rng = super::thread_rng();
        for _ in 0..N_REPLICAS {
            let smpl = super::sample(
                0,
                10,
                3.5,
                -3.0,
                -2.0,
                &|param| super::power_law::PowerLaw::with_value(param),
                &mut rng,
            );
            assert!(smpl.0.len() <= 4);
        }
    }
}
