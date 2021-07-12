# ustat
A micro-statistics program (like ministat) written in Rust which computes the sum, mean, median, min, max and standard deviation of a set of input files

```
Usage:
  ./target/release/ustat [OPTIONS]

Compute statistics for the given input file. 

Optional arguments:
  -h,--help             Show this help message and exit
  -i,--input INPUT      The input file to compute statistics for. (Use stdin if not specified)
  -c,--column COLUMN    The column to extract data from (Starts from 0, default is 0)
  -d,--delimiter DELIMITER
                        The text delimiter to use between columns. (Default: ,)
  -s,--skip-header      Skip the first line of the input file. (Default: False)
```
