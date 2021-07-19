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

fn _read_line(
    line: &str,
    line_buffer: &mut String,
    buffer: &mut std::vec::Vec<f64>,
    column: usize,
    delimiter: char,
    line_num: i32,
) -> io::Result<()> {
    let mut in_quote_context = false;
    let mut active_column = 0;
    let mut last_char = '_';
    for c in line.chars() {
        // Handle a quote scenario
        if last_char == '"' {
            if c == '"' {
                // This quote and the last quote are escaped, so we just continue as planned
                last_char = '_'; // reset last char to avoid triggering this condition again
                continue;
            }
            // The previous quote wasn't escaped, and we need to toggle the quote context
            in_quote_context = !in_quote_context; // And we shift the quote state
            last_char = '_';
        }

        // Handle a standard character
        if c == '"' {
            // Defer execution in the next timestep
            last_char = c;
            continue;
        } else if !in_quote_context && c == delimiter {
            active_column += 1;
            if active_column > column {
                break; // Early termination of parsing, if we already have what we want.
            }
            continue;
        } else if active_column == column {
            line_buffer.push(c); // This line is part of the numerics.
        }
        last_char = c;
    }
    buffer.push(line_buffer.trim_end().parse::<f64>().unwrap_or_else(|_| {
        panic!(
            "Invalid numeric value found on line {}, column {}, \"{}\"",
            line_num, column, line_buffer
        )
    }));

    Ok(())
}

pub fn read_from_stdin(
    buffer: &mut std::vec::Vec<f64>,
    delimiter: char,
    column: usize,
    skip_header: bool,
) -> io::Result<()> {
    let stdin = io::stdin();
    let lock = stdin.lock();
    let mut reader = BufReader::new(lock);
    let mut line = String::with_capacity(1024);
    let mut line_buffer = String::with_capacity(32);
    let mut line_number = 0;
    if skip_header {
        reader.read_line(&mut line).unwrap();
        line.clear();
        line_number += 1;
    }
    loop {
        if skip_fail!(reader.read_line(&mut line)) <= 0 {
            break;
        }
        _read_line(
            &line,
            &mut line_buffer,
            buffer,
            column,
            delimiter,
            line_number,
        )
        .unwrap();
        line.clear();
        line_buffer.clear();
        line_number += 1;
    }

    Ok(())
}

pub fn read_from_file(
    file_path: &String,
    buffer: &mut std::vec::Vec<f64>,
    delimiter: char,
    column: usize,
    skip_header: bool,
) -> io::Result<()> {
    // Read the elements into the buffer
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::with_capacity(1024);
    let mut line_buffer = String::with_capacity(32);
    let mut line_number = 0;
    if skip_header {
        reader.read_line(&mut line).unwrap();
        line.clear();
        line_number += 1;
    }
    loop {
        if skip_fail!(reader.read_line(&mut line)) <= 0 {
            break;
        }
        _read_line(
            &line,
            &mut line_buffer,
            buffer,
            column,
            delimiter,
            line_number,
        )
        .unwrap();
        line.clear();
        line_buffer.clear();
        line_number += 1;
    }
    Ok(())
}
