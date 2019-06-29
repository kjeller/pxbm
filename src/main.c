#include "pxbm.h"
#include <stdio.h>
#include <stdlib.h>

// =========================
// Bitmaps used for testing
#include "xbm/loink.xbm"
#include "xbm/test_9x4.xbm"
#include "xbm/test_5x4.xbm"

// === Test xbm from .xbm files ===
XBM loink = {
	loink_bits,
	loink_width,
	loink_height
};

XBM test_5x4 = {
	test_bits,
	test_width,
	test_height
};

XBM test_9x4 = {
	test_9x4_bits,
	test_9x4_width,
	test_9x4_height
};
// ==============================

int main(int argc, char *argv[]) {
	if(argc < 4) {
		printf("Error: Missing arguments!");
		return(1);
	}
	// TODO: Use filename from argv when argp is implemented
	uint8_t *data = read_file("xbm/loink.xbm");

	if(data) {
		XBM *xptr = decode_xbm(data);
		if(xptr) {
			//Handle input as <R> <G> <B>
			print_xbm(xptr, atoi(argv[1]), atoi(argv[2]), atoi(argv[3]));
		} else {
			printf("Error: Could not decode file. Invalid data.");
		}
	} else {
		printf("Error: Could not read file. Corrupted or missing?");
		return(2);
	}
}
