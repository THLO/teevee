extern crate clap;

use clap::{Arg, App, ArgGroup};
use chrono::{NaiveDate, Datelike, Local};
use teevee::{TVShow, Episode};
use teevee::parsing;

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
                .args(&vec!["start", "past"]))
        .get_matches();

    // Get the end date if supplied. Otherwise, it is set to the current date:
    let today = Local::today();
    let mut end_date = NaiveDate::from_ymd(today.year(), today.month(), today.day());
    if let Some(date) = matches.value_of("end") {
        match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(provided_date) => end_date = provided_date,
            Err(_) => {
                println!("Error: The end date must be specified in the format [YEAR]-[MONTH]-[DAY].");
                return
            }
        };
    }
    println!("Value for end date: {}", end_date);
    // Get the start date:
    let mut start_date = NaiveDate::from_ymd(today.year(), today.month(), today.day());
    if let Some(date) = matches.value_of("start") {
        match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(provided_date) => start_date = provided_date,
            Err(_) => {
                println!("Error: The start date must be specified in the format [YEAR]-[MONTH]-[DAY].");
                return
            }
        };
    }
    if let Some(years_months_days) = matches.value_of("past") {
        match parsing::parse_subtract_from_date(years_months_days, end_date) {
            Ok(date) => start_date = date,
            Err(_) => {
                println!("Error: The interval must be specified in the format [NUMBER] [d(ays) | m(onths) | y(ears)].");
                return
            }
        };
    }
    println!("Value for start date: {}", start_date);
}
