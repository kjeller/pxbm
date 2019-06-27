#include "xbmprint.h"
#include <stdlib.h>
#include <stdio.h>

// =========================
// Bitmaps used for testing
#include "xbm/loink.xbm"
#include "xbm/test_9x4.xbm"
#include "xbm/test_5x4.xbm"

// === Test xbm from .xbm files ===
XMB loink = {
	loink_bits,
	loink_width,
	loink_height
};

XMB test_5x4 = {
	test_bits,
	test_width,
	test_height
};

XMB test_9x4 = {
	test_9x4_bits,
	test_9x4_width,
	test_9x4_height
};
// ==============================

int main(int argc, char *argv[]) {
	if(argc < 4) {
		printf("Missing arguments!");
		return(1);
	}

	//Handle input as <R> <G> <B>
	print_xmb(&loink, atoi(argv[1]), atoi(argv[2]), atoi(argv[3]));
}
