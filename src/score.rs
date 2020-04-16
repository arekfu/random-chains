use super::sample::{Sample, Samples};
use std::fmt;

#[derive(Debug)]
pub struct Observable {
    value: f64,
}

#[derive(Debug)]
pub struct Score {
    pub mean: f64,
    pub std_err: f64,
}

impl Observable {
    pub fn from(sample: &Sample) -> Observable {
        let value = sample.0.iter().sum::<f64>() / sample.0.len() as f64;
        Observable { value }
    }
}

impl Score {
    pub fn from(samples: &Samples) -> Score {
        // use Welford's online algorithm
        let (mean, sum_sq_diff, count) = samples
            .0
            .iter()
            .map(|sample| Observable::from(sample))
            .fold((0.0, 0.0, 0), |acc, x| {
                let (mean_1, sum_sq_diff_1, n_1) = acc;
                let n = n_1 + 1;
                let n_f64 = n as f64;
                let delta = x.value - mean_1;
                let mean = mean_1 + delta / n_f64;
                let new_delta = x.value - mean;
                let sum_sq_diff = sum_sq_diff_1 + delta * new_delta;
                (mean, sum_sq_diff, n)
            });
        let std_err2 = sum_sq_diff / (count * (count - 1)) as f64;
        assert!(std_err2 >= 0.0, "std_err2 = {} < 0.0!", std_err2);
        let std_err = std_err2.sqrt();

        Score { mean, std_err }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ± {}", self.mean, self.std_err)
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::*;

    #[test]
    fn check_welford_trivial() {
        let sample0 = super::Sample(vec![0.]);
        let sample1 = super::Sample(vec![0.]);
        let samples = super::Samples(vec![sample0, sample1]);
        let score = super::Score::from(&samples);
        assert_f64_near!(score.mean, 0.0, 2);
        assert_f64_near!(score.std_err, 0.0, 2);
    }

    #[test]
    fn check_welford_2() {
        let sample0 = super::Sample(vec![0.]);
        let sample1 = super::Sample(vec![1.]);
        let samples = super::Samples(vec![sample0, sample1]);
        let score = super::Score::from(&samples);
        assert_f64_near!(score.mean, 0.5, 2);
        assert_f64_near!(score.std_err, 0.5, 2);
    }

    #[test]
    fn check_welford_3() {
        let sample0 = super::Sample(vec![0.]);
        let sample1 = super::Sample(vec![1.]);
        let sample2 = super::Sample(vec![2.]);
        let samples = super::Samples(vec![sample0, sample1, sample2]);
        let score = super::Score::from(&samples);
        assert_f64_near!(score.mean, 1.0, 2);
        assert_f64_near!(score.std_err, (1.0 / 3.0 as f64).sqrt(), 2);
    }

    #[test]
    fn check_welford_3_again() {
        let sample0 = super::Sample(vec![-2.]);
        let sample1 = super::Sample(vec![1.]);
        let sample2 = super::Sample(vec![2.]);
        let samples = super::Samples(vec![sample0, sample1, sample2]);
        let score = super::Score::from(&samples);
        assert_f64_near!(score.mean, 1. / 3. as f64, 2);
        assert_f64_near!(score.std_err, (13.0 as f64).sqrt() / 3.0 as f64, 2);
    }

    #[test]
    fn check_observable() {
        let sample = super::Sample(vec![0.]);
        let obs = super::Observable::from(&sample);
        assert_eq!(obs.value, 0.0);

        let sample = super::Sample(vec![0.0, 4.0]);
        let obs = super::Observable::from(&sample);
        assert_eq!(obs.value, 2.0);

        let sample = super::Sample(vec![0.0, 4.0, 2.0]);
        let obs = super::Observable::from(&sample);
        assert_eq!(obs.value, 2.0);
    }
}
