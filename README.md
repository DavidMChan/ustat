# ustat
A micro-statistics program (like ministat) written in Rust which computes the sum, mean, median, min, max, standard deviation and a one-way ANOVA of a set of input files.

```
Usage:
  ustat [OPTIONS] [FILE ...]

Compute statistics for the given input file(s).

Positional arguments:
  file                  The input file(s) to compute statistics for (Use stdin
                        if not specified)

Optional arguments:
  -h,--help             Show this help message and exit
  -c,--column COLUMN    The column(s) to extract data from (Default: 0 for all
                        files, runs from 0 to ...)
  -d,--delimiter DELIMITER
                        The text delimiter character to use between columns
                        (Default: ',')
  -s,--skip-header      Skip the first line of the input file (Default: False)
  --no-anova            Don't compute ANOVA for the input files (one file per
                        population) (Default: False)
  -V,--version          Show version
```

## Examples / Recipes

Compute basic statistics for column 0 of a CSV:
```bash
> cat file.csv | ustat 
```

Compute basic statics and an ANOVA for three files (column 0 for each CSV file):
```bash
> ustat file_1.csv file_2.csv file_3.csv
```

Compute basic statistics for a TSV file:
```bash
> ustat -d '\t' file_1.tsv
```

Compute statistics for column 0 of file_1.csv and column 3 of file_2.csv:
```bash
> ustat -c 0 -c 3 file_1.csv file_2.csv
```
