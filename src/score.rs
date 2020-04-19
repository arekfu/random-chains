use super::sample::Sample;
use super::config::ScoreMethod;
use std::fmt;
use std::iter::Iterator;

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
    pub fn new(value: f64) -> Observable {
        Observable { value }
    }
    pub fn score(sample: &Sample) -> Observable {
        let value = sample.0.iter().sum::<f64>() / sample.0.len() as f64;
        Observable { value }
    }
}

impl Score {
    pub fn welford<I>(obs: I) -> Score
    where
        I: IntoIterator<Item = Observable>,
    {
        // use Welford's online algorithm
        let (mean, sum_sq_diff, count) = obs.into_iter().fold((0.0, 0.0, 0), |acc, x| {
            let (mean_1, sum_sq_diff_1, n_1) = acc;
            let n = n_1 + 1;
            let delta = x.value - mean_1;
            let mean = mean_1 + delta / n as f64;
            let new_delta = x.value - mean;
            let sum_sq_diff = sum_sq_diff_1 + delta * new_delta;
            (mean, sum_sq_diff, n)
        });
        assert!(count > 1);
        let std_err2 = sum_sq_diff / (count as f64 * (count - 1) as f64);
        assert!(std_err2 >= 0.0, "std_err2 = {} < 0.0!", std_err2);
        let std_err = std_err2.sqrt();

        Score { mean, std_err }
    }

    pub fn squares<I>(obs: I) -> Score
    where
        I: IntoIterator<Item = Observable>,
    {
        let (sum1, sum2, count) = obs.into_iter().fold((0.0, 0.0, 0), |acc, x| {
            let (sum1, sum2, n) = acc;
            (sum1 + x.value, sum2 + x.value * x.value, n + 1)
        });
        let mean = sum1 / (count as f64);
        let std_err2 = ((sum2 / count as f64) - mean * mean) / ((count - 1) as f64);
        assert!(std_err2 >= 0.0, "std_err2 = {} < 0.0!", std_err2);
        let std_err = std_err2.sqrt();

        Score { mean, std_err }
    }

    pub fn score<I>(obs: I, method: &ScoreMethod) -> Score
    where
        I: IntoIterator<Item = Observable>,
    {
        match method {
            ScoreMethod::Welford => Self::welford(obs),
            ScoreMethod::Squares => Self::squares(obs),
        }
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
        let score = super::Score::welford(
            [super::Sample(vec![0.]), super::Sample(vec![0.])]
                .iter()
                .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 0.0, 2);
        assert_f64_near!(score.std_err, 0.0, 2);
    }

    #[test]
    fn check_welford_2() {
        let score = super::Score::welford(
            [super::Sample(vec![0.]), super::Sample(vec![1.])]
                .iter()
                .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 0.5, 2);
        assert_f64_near!(score.std_err, 0.5, 2);
    }

    #[test]
    fn check_welford_3() {
        let score = super::Score::welford(
            [
                super::Sample(vec![0.]),
                super::Sample(vec![1.]),
                super::Sample(vec![2.]),
            ]
            .iter()
            .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 1.0, 2);
        assert_f64_near!(score.std_err, (1.0 / 3.0 as f64).sqrt(), 2);
    }

    #[test]
    fn check_welford_3_again() {
        let score = super::Score::welford(
            [
                super::Sample(vec![-2.]),
                super::Sample(vec![1.]),
                super::Sample(vec![2.]),
            ]
            .iter()
            .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 1. / 3. as f64, 2);
        assert_f64_near!(score.std_err, (13.0 as f64).sqrt() / 3.0 as f64, 2);
    }

    #[test]
    fn check_squares_trivial() {
        let score = super::Score::squares(
            [super::Sample(vec![0.]), super::Sample(vec![0.])]
                .iter()
                .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 0.0, 2);
        assert_f64_near!(score.std_err, 0.0, 2);
    }

    #[test]
    fn check_squares_2() {
        let score = super::Score::squares(
            [super::Sample(vec![0.]), super::Sample(vec![1.])]
                .iter()
                .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 0.5, 2);
        assert_f64_near!(score.std_err, 0.5, 2);
    }

    #[test]
    fn check_squares_3() {
        let score = super::Score::squares(
            [
                super::Sample(vec![0.]),
                super::Sample(vec![1.]),
                super::Sample(vec![2.]),
            ]
            .iter()
            .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 1.0, 2);
        assert_f64_near!(score.std_err, (1.0 / 3.0 as f64).sqrt(), 2);
    }

    #[test]
    fn check_squares_3_again() {
        let score = super::Score::squares(
            [
                super::Sample(vec![-2.]),
                super::Sample(vec![1.]),
                super::Sample(vec![2.]),
            ]
            .iter()
            .map(super::Observable::score),
        );
        assert_f64_near!(score.mean, 1. / 3. as f64, 2);
        assert_f64_near!(score.std_err, (13.0 as f64).sqrt() / 3.0 as f64, 2);
    }

    #[test]
    fn check_observable() {
        let sample = super::Sample(vec![0.]);
        let obs = super::Observable::score(&sample);
        assert_eq!(obs.value, 0.0);

        let sample = super::Sample(vec![0.0, 4.0]);
        let obs = super::Observable::score(&sample);
        assert_eq!(obs.value, 2.0);

        let sample = super::Sample(vec![0.0, 4.0, 2.0]);
        let obs = super::Observable::score(&sample);
        assert_eq!(obs.value, 2.0);
    }
}
