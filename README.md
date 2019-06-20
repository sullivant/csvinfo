## csv_utils

A small utility to display some metadata about the fields available in a "csv" (or otherwise delimited) data file.  This came about because I got tired of trying to determine the field lengths in a source data file that was very large and also wanted to learn some Rust.

### Usage
```bash
$ csv_utils --help
CSV Utils 0.0.2
Thomas Sullivan <sullivan.t@gmail.com>
Shows some info on CSV files.

USAGE:
    csv_fields [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delim <delim>    Sets the field delimiter to use, default is ','

ARGS:
    <file>    Sets the input file to use

$ csv_utils sample.csv
Field: 1 = 6
Field: 2 = 3
Field: 3 = 9
```

### TODO
- [X] Use the crate "clap" as a way to pass CLI parameters
- [ ] Allow for any single char passed as parameter

