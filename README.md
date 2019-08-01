## csvinfo

A small utility to display some metadata about the fields available in a "csv" (or otherwise delimited) data file.  This came about because I got tired of trying to determine the field lengths in a source data file that was very large and also wanted to learn some Rust.

[![Build Status](https://travis-ci.com/sullivant/csvinfo.svg?branch=master)](https://travis-ci.com/sullivant/csvinfo)

### Reminder

This is just here while I tinker with some "applied" learning of Rust - don't trust it to do what it says on the tin.

You should really probably be using XSV: https://github.com/BurntSushi/xsv but, I am taking examples in idea from there and implementing them here because, again, this is a "hands on" place for me to do some learnin'.

### Installation
```
1. git clone git@github.com:sullivant/csvinfo.git
2. cd csvinfo
3. cargo build --release
4. cp ./target/release/csvinfo ~/bin (or somewhere in your path)
```

### Usage
```bash
$ ./csvinfo --help
CSV Utils 0.4.3
Thomas Sullivan <sullivan.t@gmail.com>
Shows some info on CSV files.

USAGE:
    csvinfo [FLAGS] [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -q, --quotes     When passed, data is quoted.
    -s, --skip       When used, skips the first record (header)
    -V, --version    Prints version information

OPTIONS:
    -d, --delim <delim>        Sets the field delimiter to use (example: -d '|'), default is ','
    -m, --max <max_records>    When provided, will stop gathering data after N records

ARGS:
    <file>    Sets the input file to use
```
### Output
```bash
$ ./csvinfo ../tmp/file_of_data.csv -d'|'
175646 records in file (| delim).
Field  Max Len  ( %int  %float %char  ) Empty?  Title
1      5        (  0.00   0.00 100.00 )          Type
2      6        ( 25.00  25.00  50.00 )         Value
```

### General Roadmap
- [X] Use the crate "clap" as a way to pass CLI parameters
- [X] Allow for any single char passed as parameter
- [X] Allow for quoted values
- [X] Allow for field names to be gathered from header data instead of "field 1, field 2..."
- [X] Test cases
- [X] Prettier looking CLI output
- [X] Which are always numeric?
- [X] Which have empty vals?
- [X] Trim extra spaces (eg: ```"Name", "Age","Location"```)
- [ ] Decide if we want to allow for mixed quoted values (some quoted, some not)
- [ ] Process escaped delimiters
- [ ] Status bar on CLI while waiting/processing
- [ ] Add more "metadata" to the output; instead of all the fields, maybe bucket them into sizes? ( wide files look odd in the results )
