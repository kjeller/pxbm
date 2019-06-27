#  ----- Constants -----

IDIR   := ../include
CC     := gcc
CFLAGS := -I $(IDIR)

ODIR   := obj

# If you're familiar with bash scripts, you may have come across patsubst before.
# ex: $(patsubst %.c, $(IDIR)/%, bar) produces: bar.c ../include/bar

_DEPS  := xbmprint.h
DEPS   := $(patsubst %, $(IDIR)/%, $(_DEPS))   

_OBJ   := main.o xbmprint.o
OBJ    := $(patsubst %,$(ODIR)/%,$(_OBJ))


# First rule in the make file is issued if no one else is declared.

# This rule will apply to all o-files, the "%.c" makes the o-files depend on the c version of it and the header file.
# The rule says that to create the o-file, make needs to compile the c-file.

# c-flag: Creates objects file for the file that is being compiled. ex: myfile.c -> myfile.o
# $@ is what is to the left of the ':', ex: pxbm: a few lines under, &@ would give pxbm
# $< Is the first item in the dependency list of creating a file. ex: pxbm: a few lines under, $< would give main.c 

pxbm: $(OBJ)
	$(CC) -o $@ $^ $(CFLAGS)


$(ODIR)/%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

.PHONY: clean

clean:
	rm -f $(ODIR)/*.o *~ core $(INCDIR)/*~
	rm ./pxbm -f

# This Makefile was created and understood thanks to http://www.cs.colby.edu/maxwell/courses/tutorials/maketutor/
# All cred to the creator of that tutorial.
