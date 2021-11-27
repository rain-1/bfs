# Usage

```
Breadth First Search the filesystem.

USAGE:
    bfs [FLAGS] [ARGS]

FLAGS:
    -h, --help       Prints help information
    -0, --print0     Use NUL delimiters instead of newlines.
    -V, --version    Prints version information

ARGS:
    <regex>    The regex to match filenames against [default: .*]
    <path>     A path to start search from [default: .]
```

# Exit codes

* 0 if it found at least one result
* 1 if it found no results

# Behavior

Filenames can contain arbitrary byte strings except for \0. They can be invalid UTF-8 text.

In the default mode there is a lossy conversion to valid UTF-8 applied before testing the regex and printing.

In the print0 mode the conversion is applied before testing the regex, but the raw filename is printed as bytes.
