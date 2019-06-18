# pxbm
Application for printing .xbm images in RGB to terminal.

Works by changing bakground color (ANSII) and printing " " (whitespace) to the terminal.
Only tested with MSYS2, but should work in all linux environment.

Compile: gcc main.c bitmap.c

#TODOs
-- Support for parsing arguments: for loading in .xbm files that are not predefined in code. (maybe argp or getopt long)

-- Support other bitmap formats (maybe netpbm?)

