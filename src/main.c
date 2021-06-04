#include "pxbm.h"
#include <stdio.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
	if(argc < 5) {
		printf("Error: Missing arguments!\n");
		printf("./pxbm <filepath> <Red> <Green> <Blue>\n");
		return(1);
	}
	// argv[1] is the filename
	char *data = read_file(argv[1]);

	if(data) {
		XBM xptr; 
		int res = parse_xbm(&xptr, data);

		if(res != 0) {
			printf("Error: Could not parse xbm data");
		}
		// Handle RGB input
		print_xbm(&xptr, atoi(argv[2]), atoi(argv[3]), atoi(argv[4]));
	} else {
		printf("Error: Could not read file. Corrupted or missing?");
		return(2);
	}
}
