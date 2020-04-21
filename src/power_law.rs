use rand_distr;

pub struct PowerLaw {
    pub exponent: f64,
}

impl PowerLaw {
    pub fn with_value(exponent: f64) -> Self {
        PowerLaw { exponent }
    }
}

impl rand_distr::Distribution<f64> for PowerLaw {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        let xi: f64 = rng.sample(rand_distr::OpenClosed01);
        let exp = 1.0 / (self.exponent + 1.0);
        xi.powf(exp)
    }
}

pub struct PowerLawCutoff {
    pub cutoff: f64,
    power_law: PowerLaw,
}

impl PowerLawCutoff {
    pub fn with_value(exponent: f64, cutoff: f64) -> Self {
        let power_law = PowerLaw::with_value(exponent);
        PowerLawCutoff { cutoff, power_law }
    }
}

impl rand_distr::Distribution<f64> for PowerLawCutoff {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        let mut accepted = false;
        let mut sampled: f64 = 0.0;
        while !accepted {
            sampled = rng.sample(&self.power_law);
            let acc_prob: f64 = (-sampled / self.cutoff).exp();
            let xi: f64 = rng.sample(rand_distr::OpenClosed01);
            if xi < acc_prob {
                accepted = true;
            }
        }
        sampled
    }
}

pub struct Compose<T, F, U>
where
    T: rand_distr::Distribution<f64>,
    U: rand_distr::Distribution<f64>,
    F: Fn(f64) -> U,
{
    param_distr: T,
    dependent_distr: F,
}

impl<T, F, U> Compose<T, F, U>
where
    T: rand_distr::Distribution<f64>,
    U: rand_distr::Distribution<f64>,
    F: Fn(f64) -> U,
{
    pub fn with_distrs(param_distr: T, dependent_distr: F) -> Self {
        Compose {
            param_distr,
            dependent_distr,
        }
    }
}

impl<T, F, U> rand_distr::Distribution<f64> for Compose<T, F, U>
where
    T: rand_distr::Distribution<f64>,
    U: rand_distr::Distribution<f64>,
    F: Fn(f64) -> U,
{
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        let param = rng.sample(&self.param_distr);
        let dependent_distr = &self.dependent_distr;
        rng.sample(dependent_distr(param))
    }
}
