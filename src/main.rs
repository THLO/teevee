extern crate clap;

use clap::{Arg, App, ArgGroup};

fn main() {
    // Extract the version:
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let matches = App::new("teevee")
                          .version(VERSION)
                          .author("Thomas Locher <thamasta@gmx.ch>")
                          .about("A simple tool to keep track of air dates of TV shows")
                          .arg(Arg::with_name("start")
                               .short("s")
                               .long("start")
                               .value_name("START_DATE")
                               .help("Sets the start date for the search (default: today)")
                               .takes_value(true))
                           .arg(Arg::with_name("end")
                                .short("e")
                                .long("end")
                                .value_name("END_DATE")
                                .help("Sets the end date for the search (default: today)")
                                .takes_value(true))
                            .arg(Arg::with_name("input")
                                 .short("i")
                                 .long("input")
                                 .value_name("INPUT_FILE")
                                 .help("Sets the input file (default: ./teevee.input)")
                                 .takes_value(true))
                            .arg(Arg::with_name("past")
                                 .short("p")
                                 .long("past")
                                 .value_name("INTERVAL")
                                 .help("Defines the search interval until the end date")
                                 .takes_value(true))
                             .arg(Arg::with_name("title")
                                  .short("t")
                                  .long("title")
                                  .help("Instructs to show the episode titles"))
                              .arg(Arg::with_name("verbose")
                                   .short("v")
                                   .long("verbose")
                                   .help("Print verbose output"))
                               .group(ArgGroup::with_name("start_past")
                                    .args(&vec!["start", "past"]))
                          .get_matches();

    // Gets a value for start_date if supplied by user, or defaults to "TODAY":
    let start_date = matches.value_of("start").unwrap_or("TODAY");
    println!("Value for start date: {}", start_date);


    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}
