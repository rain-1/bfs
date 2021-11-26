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
