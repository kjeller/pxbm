#include "bitmap.h"

// Bitmaps used for testing
#include "xbm/loink.xbm"
#include "xbm/test_9x4.xbm"
#include "xbm/test_5x4.xbm"

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

int main(int argc, char *argv[]) {
	print_xmb(&loink, 255, 255, 255);
}