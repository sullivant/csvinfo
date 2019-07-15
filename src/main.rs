#[macro_use]
extern crate clap;
extern crate csv;

use clap::{App, Arg};
use std::error::Error;
use std::process;

// Struct and the impl hold the necessary info about the fields
struct Field {
    pos: i32,
    max_len: i32,
    title: String,
    types: (i32, i32, i32), // int, float, char
}
impl Field {
    // Just a returns a pretty string for output purposes
    pub fn to_string(&self) -> String {
        format!(
            "{}\t{}\t({})\t\t{}",
            self.pos + 1,
            self.max_len,
            self.profile(),
            self.title,
        )
    }

    // Returns a profile in % based on the types tuple
    pub fn profile(&self) -> String {
        let sum: f64 = (self.types.0 + self.types.1 + self.types.2) as f64;
        format!(
            "{:.4}, {:.4}, {:.4}",
            (self.types.0 as f64 / sum) * 100.0,
            (self.types.1 as f64 / sum) * 100.0,
            (self.types.2 as f64 / sum) * 100.0
        )
    }
}

// Does what you think it does.
pub fn output_header() -> String {
    String::from("Field\tMax\tTypes % (i, f, c)\t\t\tTitle")
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn get_parameters<'a>() -> clap::ArgMatches<'a> {
    App::new("CSV Utils")
        .version("0.2.0")
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
        .arg(
            Arg::with_name("quotes")
                .short("q")
                .long("quotes")
                .help("When passed, data is quoted."),
        )
        .get_matches()
}

fn run() -> Result<(), Box<Error>> {
    // Contains the parameters passed to the application
    let matches = get_parameters();

    // Find the file path as passed
    let file_path = matches.value_of("file").unwrap();

    // Determine the delimiter
    let delim: char = match matches.value_of("delim").unwrap_or(",") {
        "\\t" => '\t',
        s => s.parse()?,
    };

    // Determine if values are quote separated
    let quotes = if matches.is_present("quotes") {
        println!("Data is quoted.");
        true
    } else {
        false
    };

    // Determine if we need to skip the header record
    let skip_header = if matches.is_present("skip") {
        println!("Skipping header record in file.");
        true
    } else {
        false
    };

    // Determine if we need to stop processing records after a certain
    // provided count
    let mut stop_after: bool = false;
    let stop_count = value_t!(matches, "max", u64).unwrap_or(100);
    if matches.is_present("max") {
        println!("Stopping after {} records", stop_count);
        stop_after = true;
    }

    // Build the CSV reader we will use with the supplied parameters
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(skip_header)
        .delimiter(delim as u8)
        .flexible(true)
        .quoting(quotes)
        .from_path(file_path)?;

    // Contains the detail about the records
    let mut record_data: Vec<Field> = Vec::new();
    let mut record_count: u64 = 0;

    let headers = rdr.headers()?.clone();

    for result in rdr.records() {
        let record = result?;
        record_count += 1;

        // Walk through each record, and start to gather the data into the vector of field data
        let mut i: i32 = 0;
        for field in record.iter() {
            let check_val: i32 = field.trim().len() as i32; // The val we will use to determine new max

            // determine if this is an integer, or a float, or a char
            let data_type = match field.trim().parse::<i32>() {
                Ok(_) => (1, 0, 0),
                Err(_) => match field.trim().parse::<f32>() {
                    Ok(_) => (0, 1, 0),
                    Err(_) => (0, 0, 1),
                },
            };

            // Match to see if we already have this field in record_data, if we do, determine if
            // the length found here is greater than the length we have already; if not, push new
            // metadata onto record_data.
            match record_data.iter().position(|ref p| i <= p.pos) {
                Some(_) => {
                    let existing: i32 = record_data.get(i as usize).unwrap().max_len;
                    if check_val > existing {
                        record_data[i as usize].max_len = check_val;
                    }

                    // Update the info about data types
                    record_data[i as usize].types.0 += data_type.0;
                    record_data[i as usize].types.1 += data_type.1;
                    record_data[i as usize].types.2 += data_type.2;
                }
                None => {
                    record_data.push(Field {
                        pos: i,
                        max_len: check_val,
                        title: headers.get(i as usize).unwrap_or("unk").trim().to_string(),
                        types: data_type,
                    });
                }
            }

            i += 1;
        }
        if stop_after && record_count == stop_count {
            println!("Hit record stop count.");
            break;
        }
    }

    println!("{} records in file ({} delim).", record_count, delim);
    println!("{}", output_header());
    for field in record_data.iter() {
        println!("{}", field.to_string());
    }

    Ok(())
}
