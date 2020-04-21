use clap::ArgMatches;
use std::fmt;

#[derive(Debug)]
pub enum ScoreMethod {
    Welford,
    Squares,
}

#[derive(Debug)]
pub struct Config {
    pub replicas: usize,
    pub min_size: usize,
    pub max_size: usize,
    pub exponent_min: f64,
    pub exponent_max: f64,
    pub budget: f64,
    pub score_method: ScoreMethod,
    pub cutoff: f64,
}

impl Config {
    pub fn new(matches: ArgMatches) -> Config {
        let replicas = matches.value_of("replicas").unwrap().parse().unwrap();
        let min_size = matches.value_of("min_size").unwrap().parse().unwrap();
        let max_size = matches.value_of("max_size").unwrap().parse().unwrap();
        let exponent_max = -matches
            .value_of("exponent_min")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let exponent_min = -matches
            .value_of("exponent_max")
            .unwrap()
            .parse::<f64>()
            .unwrap();
        let budget = matches.value_of("budget").unwrap().parse().unwrap();
        let score_method_str = matches
            .value_of("score_method")
            .unwrap()
            .to_ascii_lowercase();
        let score_method = match score_method_str.as_ref() {
            "welford" => ScoreMethod::Welford,
            "squares" => ScoreMethod::Squares,
            _ => panic!("Unrecognized score method: {}", score_method_str),
        };
        let cutoff = matches.value_of("cutoff").unwrap().parse::<f64>().unwrap();
        Config {
            replicas,
            min_size,
            max_size,
            exponent_min,
            exponent_max,
            budget,
            score_method,
            cutoff,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
