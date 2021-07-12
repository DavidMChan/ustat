// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use log::warn;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

pub fn read_from_stdin(
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

pub fn read_from_file(
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
