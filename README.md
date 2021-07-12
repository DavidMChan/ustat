# ustat
A micro-statistics program (like ministat) written in Rust

Usage:
  ./target/release/ustat [OPTIONS]

Compute statistics for the given input file. (Use stdin if not specified)

Optional arguments:
  -h,--help             Show this help message and exit
  -i,--input INPUT      The input file to compute statistics for.
  -c,--column COLUMN    The column to extract data from.
  -d,--delimiter DELIMITER
                        The text delimiter to use between columns.
  -s,--skip-header      Skip the first line of the input file.
