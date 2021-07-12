# ustat
A micro-statistics program (like ministat) written in Rust which computes the sum, mean, median, min, max and standard deviation of a set of input files.

```
Usage:
  ustat [OPTIONS] [FILE ...]

Compute statistics for the given input file.

Positional arguments:
  file                  The input file(s) to compute statistics for (Use stdin
                        if not specified)

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--column COLUMN    The column to extract data from (Default: 0, runs from
                        0 to ...)
  -d,--delimiter DELIMITER
                        The text delimiter to use between columns (Default:
                        ',')
  -s,--skip-header      Skip the first line of the input file (Default: False)
```
