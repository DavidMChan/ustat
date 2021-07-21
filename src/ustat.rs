// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

extern crate argparse;
#[macro_use]
extern crate prettytable;
extern crate quickersort;

use argparse::{ArgumentParser, Collect, Print, Store, StoreTrue};
use log::LevelFilter;
use prettytable::Table;

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
    let mut columns: Vec<usize> = vec![0];
    let mut delimiter = ",".to_string();
    let mut skip_header = false;
    let mut dont_compute_anova = false;
    let mut input_files: Vec<String> = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Compute statistics for the given input file.");
        ap.refer(&mut columns).add_option(
            &["-c", "--column"],
            Collect,
            "The column(s) to extract data from (Default: 0 for all files, runs from 0 to ...)",
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
        ap.refer(&mut dont_compute_anova).add_option(
            &["--no-anova"],
            StoreTrue,
            "Don't compute ANOVA for the input files (one file per population) (Default: False)",
        );
        ap.refer(&mut input_files).add_argument(
            "file",
            Collect,
            "The input file(s) to compute statistics for (Use stdin if not specified)",
        );
        ap.add_option(
            &["-V", "--version"],
            Print((&"ustat version 0.2.2"[..]).to_string()),
            "Show version",
        );
        ap.parse_args_or_exit();
    }

    // Parse the delimiter to a character
    let delimiter_char = utils::parse_delimiter_from_string(delimiter);

    // Hnadle the case where there are multiple different columns.
    if input_files.len() == 0 && columns.len() <= 1 {
        columns.push(columns[0]);
    } else if input_files.len() != columns.len() && columns.len() == 1 {
        while input_files.len() > columns.len() {
            columns.push(columns[0]);
        }
    } else if input_files.len() != columns.len() {
        panic!("{} column indices passed, but there are {} files. Pass either 1 column index, or a column for each input file.", columns.len(), input_files.len());
    }

    // Load the data from the input files
    println!("{} {}", input_files.len(), columns.len());
    let mut all_buffers = Vec::new();
    if input_files.len() > 0 {
        for (input_file, column) in input_files.iter().zip(columns) {
            let mut buffer = Vec::new();
            io::read_from_file(
                &input_file,
                &mut buffer,
                delimiter_char,
                column,
                skip_header,
            )
            .expect(&format!("Could not read file: {}, Error", input_file));
            all_buffers.push(buffer);
        }
    } else {
        let mut buffer = Vec::new();
        io::read_from_stdin(&mut buffer, delimiter_char, columns[0], skip_header)
            .expect("Error reading from stdin");
        all_buffers.push(buffer);
    }

    // Compute the statistics, and print a nice output table.
    let mut table = Table::new();
    table.add_row(row![
        "File", "Lines", "Sum", "Mean", "Median", "Stddev", "Min", "Max"
    ]);
    if input_files.len() > 0 {
        for (fname, buffer) in input_files.iter().zip(&mut all_buffers) {
            // Compute the statistics independently for each of the buffers
            quickersort::sort_by(&mut buffer[..], &|a, b| a.partial_cmp(b).unwrap());
            let (mean, count, median, std_dev, accum, min, max) = stats::compute_stats(&buffer);
            table.add_row(row![fname, count, accum, mean, median, std_dev, min, max]);
        }
    } else {
        // Only comput statistics for the first buffer with name stdin
        let mut data: Vec<f64> = all_buffers.iter().flat_map(|s| s.iter()).copied().collect();
        quickersort::sort_by(&mut data[..], &|a, b| a.partial_cmp(b).unwrap());
        let (mean, count, median, std_dev, accum, min, max) = stats::compute_stats(&data);
        table.add_row(row!["stdin", count, accum, mean, median, std_dev, min, max]);
    }
    table.printstd();

    if !dont_compute_anova && input_files.len() > 1 {
        stats::compute_anova(&mut all_buffers);
    }
}
