#include "xbmprint.h"
#include <stdio.h>
#include <string.h>

/**
 * Reads xbm file and converts it into a XBM_struct defined in xbmprint.h
 * This is so that the xbm file does not need to be pre-compiled.
 */
void decode_xbm(char **filename) {
  // find width and height from file, search for "_width" and "_height"
  // strtol(string, NULL, 10)

  // go to start of image data

  // extract juicy data from array, hexadecimal val to int
  // with (int)strtol(string, NULL, 0); if '0x' is used

}

/**
 * Prints xmb picture to console by reading bytewise (the bits) line by line
 * and changing background color (ANSI) for every bit that is 'highlighted'.
 */
void print_xmb(XMB *p, unsigned char r, unsigned char g, unsigned char b) {

  // Width of bitmap in bytes
  int w_bytes;
  w_bytes = (p->width + 7) / 8;

  for(int i = 0; i < p->height; i++) {
    for(int j = 0; j < w_bytes; j++) {
      // Get current byte with offset
      char byte = p->data[i * w_bytes + j];

      // Loop through the bits
      for(int k = 0; k < 8; k++) {
        if(byte & (1<<k))
          printf("\033[48;2;%d;%d;%dm  ", r, g, b);
        else
          printf("\033[48;2;0;0;0m  ");
      }
    }
    printf("\n");
  }
  printf("\033[48;2;0;0;0m  ");
}
