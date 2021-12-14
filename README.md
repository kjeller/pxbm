# pxbm
Application for printing .xbm images (RGB) for ANSI based consoles.

Works by decoding .xbm file, and printing it bytewise, by changing background color (ANSI color codes) 
and printing " " (whitespace) to the terminal/shell.
Bits that are highlighted in .xbm file will be colored and the 0's will be blanks.
As of now, only the highlighted bits may be colored.

![Alt text](https://i.imgur.com/D24CIG5.png "XBM print in action")

# Compile
Run makefile with ```make```

Note: The execution time is way faster with a precompiled .xbm file. 
The complexity will be higher for decoding and printing.
There are test files which shows how to compile with your own .xbm files.
An example of this can be seen in the Usage section.

# Usage
To run: 
``` 
pxbm [options] <filepath> [R] [G] [B]
which will look like this for:
./pxbm xbm/loink.xbm 255 255 255

```

# TODOs

-- Add support for other bitmap formats (bitmap?)

