# pxbm

`pxbm` is used for printing bitmap images on ANSI-based shell.
It works by changing the background color (ANSI color codes) and printing " " (whitespace) to stdout. 

[Originally written in C](https://github.com/kjeller/pxbm/tree/c_impl).

Supported formats:
| Format | ASCII (plain) | Binary (raw) | Extension | RGB input flag |
|--------------|-----------|------------|-----------|-----------|
| X BitMap     | Yes       | No         | .xbm      | Yes |
| Portable BitMap | Yes (P1)  | Yes (P4)   | .pbm   | Yes |
| Portable GrayMap | Yes (P2)  | Yes (P5)   | .pgm | No |
| Portable PixMap | Yes (P3)  | Yes (P6)   | .pgm | No


## Example XBM print
![Alt text](img/loink_xbm.png "XBM print in action")

# Compile
`cargo build`

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

