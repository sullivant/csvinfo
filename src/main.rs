#[macro_use]
extern crate clap;
extern crate csv;

use clap::{App, Arg};
use std::error::Error;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let matches = App::new("CSV Utils")
        .version("0.1.0")
        .author("Thomas Sullivan <sullivan.t@gmail.com>")
        .about("Shows some info on CSV files.")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("delim")
                .short("d")
                .long("delim")
                .help("Sets the field delimiter to use, default is ','")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("skip")
                .short("s")
                .long("skip")
                .help("When used, skips the first record (header)"),
        )
        .arg(
            Arg::with_name("max_records")
                .long("max")
                .short("m")
                .help("When provided, will stop gathering data after N records")
                .takes_value(true),
        )
        .get_matches();

    // Find the file path as passed
    let file_path = matches.value_of("file").unwrap();
    let delim: u8 = *matches
        .value_of("delim")
        .unwrap_or(",")
        .as_bytes()
        .first()
        .unwrap_or(&b',');

    // Determine if we need to skip the header record
    let mut skip_header: bool = false;
    if matches.is_present("skip") {
        println!("Skipping header record in file.");
        skip_header = true;
    }

    // Determine if we need to stop processing records after a certain
    // provided count
    let mut stop_after: bool = false;
    let stop_count = value_t!(matches, "max", u64).unwrap_or(100);
    if matches.is_present("max") {
        println!("Stopping after {} records", stop_count);
        stop_after = true;
    }

    // Build the CSV reader we will use
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(skip_header)
        .delimiter(delim)
        .flexible(true)
        .from_path(file_path)?;

    let mut rec_lengths: Vec<(i32, i32)> = Vec::new();
    let mut rec_count: u64 = 0;

    for result in rdr.records() {
        let record = result?;
        rec_count = rec_count + 1;

        let mut i: i32 = 0;
        for field in record.iter() {
            let check_val: i32 = field.len() as i32;

            match rec_lengths.iter().position(|ref p| i <= p.0) {
                Some(_) => {
                    let existing: i32 = rec_lengths.get(i as usize).unwrap().1;
                    if check_val > existing {
                        rec_lengths[i as usize] = (i, check_val);
                    }
                }
                None => {
                    rec_lengths.push((i, field.len() as i32));
                }
            }

            i = i + 1;
        }
        if stop_after && rec_count == stop_count {
            println!("Hit record stop count.");
            break;
        }
    }

    for rec_tup in rec_lengths.iter() {
        println!("Field: {} = {}", rec_tup.0 + 1, rec_tup.1);
    }
    println!("Total records in file: {}", rec_count);

    Ok(())
}
