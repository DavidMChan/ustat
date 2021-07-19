// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate argparse;

use argparse::{ArgumentParser, Collect, Store, StoreTrue};
use log::LevelFilter;

mod io;
mod logging;
mod stats;
mod utils;

fn main() {
    // Initialize logging
    static LOGGER: logging::SimpleLogger = logging::SimpleLogger;
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("Unable to initialize logger");

    // Setup the argument parsing
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
            "The text delimiter character to use between columns (Default: ',')",
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

    // Parse the delimiter to a character
    let delimiter_char = utils::parse_delimiter_from_string(delimiter);
    eprintln!("Delimiter is '{}', column is '{}'", delimiter_char, column);

    // Load the data from the input files
    let mut all_buffers = std::vec::Vec::new();
    // let mut buffer = std::vec::Vec::new();
    if input_files.len() > 0 {
        for input_file in input_files.iter() {
            let mut buffer = std::vec::Vec::new();
            io::read_from_file(
                &input_file,
                &mut buffer,
                delimiter_char,
                column,
                skip_header,
            )
            .expect(&format!("Error reading from file {}", input_file));
            all_buffers.push(buffer);
        }
    } else {
        let mut buffer = std::vec::Vec::new();
        io::read_from_stdin(&mut buffer, delimiter_char, column, skip_header)
            .expect("Error reading from stdin");
        all_buffers.push(buffer);
    }

    // Collect all data using flat map
    let mut data: std::vec::Vec<f64> = all_buffers.iter().flat_map(|s| s.iter()).copied().collect();

    // Sort the buffer, and compute the statistics
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (mean, count, median, std_dev, accum, min, max) = stats::compute_stats(&data);

    // Print the results
    println!("Sum: {}", accum);
    println!("Count: {}", count);
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Stddev: {}", std_dev);
    println!("Min: {}", min);
    println!("Max: {}", max);
}
