# pxbm
Application for printing .xbm images (RGB) for ANSI based consoles.

Works by changing bakground color (ANSI color codes) and printing " " (whitespace) to the terminal.
Bits that are highlighted in .xbm file will be colored and the 0's will be blanks. As of now, only the highlighted bits
may be colored.

Only tested with MSYS2, but should work for every ANSI-based console.

Compile: gcc main.c bitmap.c

# Usage
-- As of now, RGB is not implemented very well


Run the executable with parameters <R> <G> <B> e.g. ```./a.out 255 255 255``` will print the picture in white. 

# TODOs
-- Support for parsing arguments: for loading in .xbm files that are not predefined in code. (maybe argp or getopt long)

-- Support other bitmap formats (maybe netpbm?)

