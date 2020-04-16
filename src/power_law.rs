use rand;
use rand_distr;

use super::power_law;

pub struct PowerLaw {
    pub exponent: f64,
    one_over_exponent_plus_1: f64,
}

impl PowerLaw {
    pub fn new(exponent: f64) -> Self {
        let one_over_exponent_plus_1 = 1.0 / (exponent + 1.0);
        PowerLaw {
            exponent,
            one_over_exponent_plus_1,
        }
    }
}

impl rand_distr::Distribution<f64> for PowerLaw {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        let xi: f64 = rng.sample(rand_distr::OpenClosed01);
        let base = xi;
        let exp = self.one_over_exponent_plus_1;
        base.powf(exp)
    }
}

pub struct VariablePowerLaw {
    exp_distr: rand_distr::uniform::Uniform<f64>,
}

impl VariablePowerLaw {
    pub fn new(exponent_min: f64, exponent_max: f64) -> Self {
        let exp_distr = rand_distr::uniform::Uniform::new_inclusive(exponent_min, exponent_max);
        VariablePowerLaw { exp_distr }
    }
}

impl rand_distr::Distribution<f64> for VariablePowerLaw {
    fn sample<R>(&self, rng: &mut R) -> f64
    where
        R: rand::Rng + ?Sized,
    {
        let exp = rng.sample(self.exp_distr);
        rng.sample(power_law::PowerLaw::new(exp))
    }
}
