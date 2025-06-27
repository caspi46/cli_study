
extern crate clap; 

use clap::{Arg, App}; 

fn main() {
    let matches = App::new("Rget")
        .version("0.1.0")
        .author("Kenny Kim <caspi4646@gmail.com>")
        .about("wget clone written in Rust for learning purpose")
        .arg(Arg::with_name("URL")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("url to download"))
        .get_matches(); 
    let url = matches.value_of("URL").unwrap(); 
    println!("{}", url); 
}
