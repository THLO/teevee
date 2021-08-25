# teevee
A simple tool to keep track of air dates of TV shows.

**Important note**: The project is currently in an early stage of development.
The usage information below states how the tool *will* work once implemented.
In other words, the tool does *not* provide the intended functionality yet.

## Usage

A source file must be prepared with the following structure:

```
[URL of TV show #1]
[URL of TV show #2]
[URL of TV show #3]
...
```

Currently, only [epguides](http://epguides.com/) URLs are supported.
Other sources of information may be added in the future.

`teevee` will look for the file `teevee.input` in the current directory.
A file with a different name, potentially under a different path, can be
provided on the command line.

Given such a file, `teevee` can be run as follows:

```
teevee [FLAGS] [OPTIONS]
```

The following `FLAGS` exist:

* `--help (-h)`: Prints help information
* `--titles (-t)`: Instructs to show the episode titles
* `--version (-V)`: Prints version information
* `--verbose (-v)`: Prints verbose output

The following `OPTIONS` exist:

* `--end (-e) END_DATE`: Sets the end date for the search
* `--input (-i) INPUT_FILE`: Sets the input file
* `--past (-p) INTERVAL`: Defines the search interval until the end date
* `--start (-s) START_DATE`: Sets the start date for the search

As mentioned above, `teevee` expects to find the file `teevee.input` in the
current working directory. Alternatively, an input can be provided using the
`--input` option.

If no end date is provided, the end date is set to the current day.
A different end date can be provided in the form `--end [YEAR]-[MONTH]-[DAY]`,
e.g., `--end 2020-04-23`.

If no start date is provided, the start date is set to the current day.
A start date can be provided in the form `--start [YEAR]-[MONTH]-[DAY]` or
as a difference to the end date in the form `--past [X][days | months |  years]`,
e.g., `--past 9days` or `--past 19months`. The use of of multiple time units
is allowed, e.g., `--past "1month 2days"`
It is further permissible to use `d`, `m`, and `y` for days, months, and years,
respectively.
