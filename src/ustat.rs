// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate argparse;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};
use log::warn;
use log::{Level, Metadata, Record};
use log::{LevelFilter, SetLoggerError};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}

macro_rules! skip_fail {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                warn!("Reading file line error: {}; skipped.", e);
                continue;
            }
        }
    };
}

fn compute_stats(buffer: &std::vec::Vec<f64>) -> (f64, usize, f64, f64, f64, f64, f64) {
    // Compute the mean, sum and standard deviation of the buffer

    let accum = buffer.iter().sum::<f64>() as f64;
    let mean = accum / buffer.len() as f64;
    let squared_differences = (buffer.iter().map(|x| (x - mean) * (x - mean))).sum::<f64>() as f64;
    let std_dev = (squared_differences / (buffer.len() - 1) as f64).sqrt();

    // Compute the median of the array
    let median = if buffer.len() % 2 == 0 {
        let midpoint = buffer.len() / 2;
        (buffer[midpoint + 1] + buffer[midpoint]) / 2.0
    } else {
        buffer[buffer.len() / 2]
    };

    let min = *buffer.first().expect("No minimum value found...");
    let max = *buffer.last().expect("No maximum value found...");

    (mean, buffer.len(), median, std_dev, accum, min, max) // Return value
}

fn read_from_stdin(
    buffer: &mut std::vec::Vec<f64>,
    delimiter: &String,
    column: usize,
    skip_header: bool,
) -> io::Result<()> {
    // Build the split regex
    let re = regex::Regex::new(&format!("(\"[^\"]*\")+|[^{0}]+", delimiter)[..]).unwrap();

    // Read the elements into the buffer
    let stdin = io::stdin();
    for line in stdin.lock().lines().skip(if skip_header { 1 } else { 0 }) {
        let read_line = skip_fail!(line);
        let vec = re
            .find_iter(&read_line)
            .filter_map(|strs| Some(strs.as_str()))
            .collect::<Vec<&str>>();
        if vec.len() < column {
            return Err(io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not enough columns in line.",
            ));
        }
        buffer.push(
            vec[column]
                .parse::<f64>()
                .expect(&format!("Failed to parse column value: {}", vec[column])[..]),
        );
    }

    Ok(())
}

fn read_from_file(
    file_path: &String,
    buffer: &mut std::vec::Vec<f64>,
    delimiter: &String,
    column: usize,
    skip_header: bool,
) -> io::Result<()> {
    // Build the split regex
    let re = regex::Regex::new(&format!("(\"[^\"]*\")+|[^{0}]+", delimiter)[..]).unwrap();

    // Read the elements into the buffer
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines().skip(if skip_header { 1 } else { 0 }) {
        let read_line = skip_fail!(line);
        let vec = re
            .find_iter(&read_line)
            .filter_map(|strs| Some(strs.as_str()))
            .collect::<Vec<&str>>();
        if vec.len() < column {
            return Err(io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not enough columns in line.",
            ));
        }
        buffer.push(
            vec[column]
                .parse::<f64>()
                .expect(&format!("Failed to parse column value: {}", vec[column])[..]),
        );
    }

    Ok(())
}

fn main() {
    // Initialize logging
    init().expect("Unable to initialize logger");

    // Setup the argument parsing
    // let mut input_file = "".to_string();
    let mut column = 0;
    let mut delimiter = ",".to_string();
    let mut skip_header = false;
    let mut input_files: std::vec::Vec<String> = std::vec::Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Compute statistics for the given input file.");
        ap.refer(&mut column).add_option(
            &["-c", "--column"],
            Store,
            "The column to extract data from (Default: 0, runs from 0 to ...)",
        );
        ap.refer(&mut delimiter).add_option(
            &["-d", "--delimiter"],
            Store,
            "The text delimiter to use between columns (Default: ',')",
        );
        ap.refer(&mut skip_header).add_option(
            &["-s", "--skip-header"],
            StoreTrue,
            "Skip the first line of the input file (Default: False)",
        );
        ap.refer(&mut input_files).add_argument(
            "file",
            Collect,
            "The input file(s) to compute statistics for (Use stdin if not specified)",
        );
        ap.parse_args_or_exit();
    }

    // Load the data from the input files
    let mut buffer = std::vec::Vec::new();
    if input_files.len() > 0 {
        for input_file in input_files.iter() {
            read_from_file(&input_file, &mut buffer, &delimiter, column, skip_header)
                .expect(&format!("Error reading from file {}", input_file));
        }
    } else {
        read_from_stdin(&mut buffer, &delimiter, column, skip_header)
            .expect("Error reading from stdin");
    }

    if buffer.len() == 0 {
        println!("No data to compute statistics for.");
        return;
    }

    // Sort the buffer, and compute the statistics
    buffer.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (mean, count, median, std_dev, accum, min, max) = compute_stats(&buffer);

    println!("Sum: {}", accum);
    println!("Count: {}", count);
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Stddev: {}", std_dev);
    println!("Min: {}", min);
    println!("Max: {}", max);
}
