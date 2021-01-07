extern crate clap;

use clap::{Arg, App, ArgGroup};
use chrono::prelude::*;
use teevee::{TVShow, Episode};

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
        .arg(Arg::with_name("titles")
            .short("t")
            .long("titles")
            .help("Instructs to show the episode titles"))
            .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Prints verbose output"))
            .group(ArgGroup::with_name("start_past")
            .args(&vec!["start", "past"])
            .required(true))
        .get_matches();

    // Get the end date if supplied. Otherwise, it is set to the current date:
    let end_date = if let Some(date) = matches.value_of("end") {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("The end date must be specified in the
            format [YEAR]-[MONTH]-[DAY].")
    } else {
        let today = Local::today();
        NaiveDate::parse_from_str(&format!("{}-{}-{}", today.year(), today.month(), today.day()),
            "%Y-%m-%d").unwrap()
    };
    println!("Value for end date: {}", end_date);
    // Get the start date:
    let start_date = match matches.value_of("start") {
        Some(date) => NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("The start date must be
            specified in the format [YEAR]-[MONTH]-[DAY]."),
        _ => match matches.value_of("start") {
            Some(past) => NaiveDate::parse_from_str(past, "%Y-%m-%d").expect("The interval must
                be specified in the format [NUMBER] [d(ays) | m(onths) | y(ears)]"),
            _ => {
                // This case cannot occur because either --start or --past must be provided:
                let today = Local::today();
                NaiveDate::parse_from_str(&format!("{}-{}-{}", today.year(), today.month(),
                today.day()), "%Y-%m-%d").unwrap()
            }
        }
    };
    println!("Value for start date: {}", start_date);
}
