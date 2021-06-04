CC 				= gcc
CFLAGS 		= -Wall -g
INCLUDES 	= include res
SRCS 			= src/main.c src/pxbm.c
OBJS 			= $(SRCS:.c=.o)
MAIN 			= bin/pxbm

#
# The following part of the makefile is generic; it can be used to 
# build any executable just by changing the definitions above and by
# deleting dependencies appended to the file from 'make depend'
#

.PHONY: depend clean

all:    $(MAIN)
	@echo  pxbm has been compiled successfully

$(MAIN): $(OBJS) 
	@mkdir -p bin
	$(CC) $(CFLAGS) $(addprefix -I, $(INCLUDES)) -o $(MAIN) $(OBJS)

# this is a suffix replacement rule for building .o's from .c's
# it uses automatic variables $<: the name of the prerequisite of
# the rule(a .c file) and $@: the name of the target of the rule (a .o file) 
# (see the gnu make manual section about automatic variables)
.c.o:
	$(CC) $(CFLAGS) $(addprefix -I, $(INCLUDES)) -c $<  -o $@

clean:
	$(RM) *.o *~ $(MAIN)

depend: $(SRCS)
	makedepend $(INCLUDES) $^
