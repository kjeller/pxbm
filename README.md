# pxbm

Originally written in C. Application for printing .xbm images (RGB) for ANSI based consoles.

Works by decoding .xbm file, and printing it bytewise, by changing background color (ANSI color codes) 
and printing " " (whitespace) to the terminal/shell.
Bits that are highlighted in .xbm file will be colored and the 0's will be blanks.
As of now, only the highlighted bits may be colored.

![Alt text](https://i.imgur.com/D24CIG5.png "XBM print in action")

# Compile & Run
`cargo build` & `cargo run`

# Usage
To run: 
``` 
Usage: pxbm <command> <filepath> [red] [green] [blue]

Arguments:
  <command>   [possible values: xbm, netpbm]
  <filepath>  File to print
  [red]       Red [default: 255]
  [green]     Green [default: 255]
  [blue]      Blue [default: 255]

Options:
  -h, --help     Print help
  -V, --version  Print version

```

# TODOs

-- Add support for other bitmap formats (bitmap?)

