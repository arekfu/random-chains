extern crate clap;
use clap::{crate_version, App, Arg};

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
                .default_value("0")
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
        .get_matches();

    let config: Config = Config::new(matches);
    run(&config);
}
