
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

// cargo run 
//     Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
//      Running `target/debug/rget`
// error: The following required arguments were not provided:
//     <URL>

// USAGE:
//     rget <URL>

// For more information try --help


// cargo run -- -h 
//     Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
//      Running `target/debug/rget -h`
// Rget 0.1.0
// Matt Gathu <mattgathu@gmail.com>
// wget clone written in Rust

// USAGE:
//     rget <URL>

// FLAGS:
//     -h, --help       Prints help information
//     -V, --version    Prints version information

// ARGS:
//     <URL>    url to download