# pxbm
Application for printing .xbm images (RGB) for ANSI based consoles.

Works by changing bakground color (ANSI color codes) and printing " " (whitespace) to the terminal.
Bits that are highlighted in .xbm file will be colored and the 0's will be blanks. As of now, only the highlighted bits
may be colored.

![Alt text](https://i.imgur.com/D24CIG5.png "XBM print in action")

Compile: ```gcc main.c pxbm.c``` or run the make file
Note: The execution time is way faster with a precompiled .xbm file. The complexity will be higher for decoding and printing.
There are test files which shows how to compile with your own .xbm files.

# Usage
Run the executable with parameters <filepath> <R> <G> <B> e.g. ```./a.out xbm/loink.xbm 255 255 255``` will print the picture in white. 

# TODOs

-- Add support for other bitmap formats (maybe netpbm?)

