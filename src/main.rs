#[macro_use]
extern crate clap;
extern crate csv;
extern crate tabular;

use clap::{App, Arg};
use std::error::Error;
use std::process;
use tabular::{Row, Table};

use csv_lib::Field;

fn main() {
    if let Err(err) = run(&mut std::io::stdout()) {
        println!("{}", err);
        process::exit(1);
    }
}

fn get_parameters<'a>() -> clap::ArgMatches<'a> {
    App::new("CSV Utils")
        .version("0.4.0")
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
                .help("Sets the field delimiter to use (example: -d '|'), default is ','")
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

fn run(mut writer: impl std::io::Write) -> Result<(), Box<Error>> {
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
        writeln!(writer, "Data is quoted.").unwrap();
        true
    } else {
        false
    };

    // Determine if we need to skip the header record
    let skip_header = if matches.is_present("skip") {
        writeln!(writer, "Skipping header record in file.").unwrap();
        true
    } else {
        false
    };

    // Determine if we need to stop processing records after a certain
    // provided count
    let mut stop_after: bool = false;
    let stop_count = value_t!(matches, "max", u64).unwrap_or(100);
    if matches.is_present("max") {
        writeln!(writer, "Stopping after {} records", stop_count).unwrap();
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

    // Iterate through each record in the file
    for result in rdr.records() {
        let record = result?;
        record_count += 1;

        // Iterate through each field, to gather the data into the vector of field data
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

            // Does this field actually have a value?
            let has_val: bool = check_val > 0;

            // Match to see if we already have this field in record_data, if we do, determine if
            // the length found here is greater than the length we have already; if not, push new
            // metadata onto record_data.
            match record_data.iter().position(|ref p| i <= p.pos) {
                Some(_) => {
                    // Determine if the current value's length is greater than the existing max_len
                    // and, if so, make the current length the new normal.
                    let existing: i32 = record_data.get(i as usize).unwrap().max_len;
                    if check_val > existing {
                        record_data[i as usize].max_len = check_val;
                    }

                    // Update the info about data types
                    record_data[i as usize].types.0 += data_type.0;
                    record_data[i as usize].types.1 += data_type.1;
                    record_data[i as usize].types.2 += data_type.2;

                    // Update the "has value" flag
                    record_data[i as usize].has_value(has_val);
                }
                None => {
                    // We have not yet seen this field, so lets just make a new one with the
                    // values we currently have
                    record_data.push(Field {
                        pos: i,
                        max_len: check_val,
                        title: headers.get(i as usize).unwrap_or("unk").trim().to_string(),
                        types: data_type,
                        has_value: has_val,
                    });
                }
            }

            i += 1;
        }
        if stop_after && record_count == stop_count {
            writeln!(writer, "Hit record stop count.").unwrap();
            break;
        }
    }

    writeln!(
        writer,
        "{} records in file ({} delim).",
        record_count, delim
    )
    .unwrap();

    let mut table = Table::new("{:<}  {:<}  ({:^} {:^} {:^}) {:^}  {:>}");

    table.add_row(
        Row::new()
            .with_cell("Field")
            .with_cell("Max Len")
            .with_cell("%int")
            .with_cell("%float")
            .with_cell("%char")
            .with_cell("Empty?")
            .with_cell("Title"),
    );
    for field in record_data.iter() {
        // Build the type profile for this field, so we can use it easier and line the column up
        let profile = field.build_profile();

        table.add_row(
            Row::new()
                .with_cell(field.pos + 1)
                .with_cell(field.max_len)
                .with_cell(format!("{:8.4}", profile.0))
                .with_cell(format!("{:8.4}", profile.1))
                .with_cell(format!("{:8.4}", profile.2))
                .with_cell(if !field.has_value {
                    "empty".to_string()
                } else {
                    "".to_string()
                })
                .with_cell(&field.title),
        );
    }
    print!("{}", table);

    Ok(())
}
