extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::result;

mod commands;

type Result<T> = result::Result<T, Box<Error>>;

fn main() {
    let matches = App::new("World Time")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("index")
                .about("rebuild the index of locations")
                .arg(
                    Arg::with_name("geonames")
                        .short("g")
                        .long("geonames")
                        .takes_value(true)
                        .required(true)
                        .help("geonames primary database file"),
                )
                .arg(
                    Arg::with_name("index")
                        .short("i")
                        .long("index")
                        .takes_value(true)
                        .required(true)
                        .help("index file to produce"),
                ),
        )
        .get_matches();

    let (subcommand, some_options) = matches.subcommand();
    let options = some_options.unwrap();

    match subcommand {
        "index" => commands::run_index(options),
        _ => panic!("unknown subcommand: {}", subcommand),
    };
}
