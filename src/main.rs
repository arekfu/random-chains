extern crate clap;
use clap::{crate_version, App, Arg};

extern crate rgsl;

mod config;
mod power_law;
mod sample;
mod score;

use config::Config;
use sample::run;

fn main() {
    let matches = App::new("random-chains")
        .version(crate_version!())
        .author("Davide Mancusi <davide.mancusi@cea.fr>")
        .about("Tests hypotheses about truncated sampling of chains")
        .arg(
            Arg::with_name("replicas")
                .short("r")
                .long("replicas")
                .takes_value(true)
                .default_value("1000")
                .help("Number of independent samples"),
        )
        .arg(
            Arg::with_name("min_size")
                .short("s")
                .long("min-size")
                .takes_value(true)
                .default_value("1")
                .help("Minimum size of each sample"),
        )
        .arg(
            Arg::with_name("max_size")
                .short("S")
                .long("max-size")
                .takes_value(true)
                .default_value("100")
                .help("Maximum size of each sample"),
        )
        .arg(
            Arg::with_name("exponent_min")
                .short("m")
                .long("exponent-min")
                .takes_value(true)
                .default_value("2.5")
                .help("Minimum exponent for the power-law distribution"),
        )
        .arg(
            Arg::with_name("exponent_max")
                .short("M")
                .long("exponent-max")
                .takes_value(true)
                .default_value("3.5")
                .help("Maximum exponent for the power-law distribution"),
        )
        .arg(
            Arg::with_name("budget")
                .short("b")
                .long("budget")
                .takes_value(true)
                .default_value("10.0")
                .help("Total computational budget"),
        )
        .arg(
            Arg::with_name("score_method")
                .long("score-method")
                .takes_value(true)
                .default_value("welford")
                .help("Score method"),
        )
        .arg(
            Arg::with_name("cutoff")
                .long("cutoff")
                .takes_value(true)
                .default_value("10.0")
                .help("Exponential cutoff"),
        )
        .get_matches();

    let config: Config = Config::new(matches);
    run(&config);
}
