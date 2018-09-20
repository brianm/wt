extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("World Time")
                          .version(env!("CARGO_PKG_VERSION"))
                          .author(env!("CARGO_PKG_AUTHORS"))
                          .about(env!("CARGO_PKG_DESCRIPTION"))
                          .get_matches();
    
}
