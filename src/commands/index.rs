extern crate clap;
use clap::ArgMatches;

pub fn run_index(matches: &ArgMatches) {       
     let index_path = matches.value_of("index").unwrap();
        let geonames_path = matches.value_of("geonames").unwrap();
        println!("indexing: index={} geonames={}", index_path, geonames_path);

}