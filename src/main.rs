extern crate csv;

use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<Error>> {
    let file_path = get_first_arg()?;
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .flexible(true)
        .from_path(file_path)?;

    let mut rec_lengths: Vec<(i32, i32)> = Vec::new();

    for result in rdr.records() {
        let record = result?;

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
    }

    for rec_tup in rec_lengths.iter() {
        println!("Field: {} = {}", rec_tup.0 + 1, rec_tup.1);
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("Expected filename as argument, but got none.")),
        Some(file_path) => Ok(file_path),
    }
}
