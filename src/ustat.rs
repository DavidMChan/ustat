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
            io::read_from_file(&input_file, &mut buffer, &delimiter, column, skip_header)
                .expect(&format!("Error reading from file {}", input_file));
        }
    } else {
        io::read_from_stdin(&mut buffer, &delimiter, column, skip_header)
            .expect("Error reading from stdin");
    }

    if buffer.len() == 0 {
        println!("No data to compute statistics for.");
        return;
    }

    // Sort the buffer, and compute the statistics
    buffer.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (mean, count, median, std_dev, accum, min, max) = stats::compute_stats(&buffer);

    // Print the results
    println!("Sum: {}", accum);
    println!("Count: {}", count);
    println!("Mean: {}", mean);
    println!("Median: {}", median);
    println!("Stddev: {}", std_dev);
    println!("Min: {}", min);
    println!("Max: {}", max);
}
