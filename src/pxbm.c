#include "pxbm.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

/**
 * From FFmpeg in ffmpeg/libavdcodec/xbmdec.c
 * download source here: https://ffmpeg.org/
 * The functions finds the key string and extracts
 * the int that is placed behind it.
 */
static int parse_str_int(const uint8_t *p, int len, const uint8_t *key)
{
    const uint8_t *end = p + len;

    for(; p<end - strlen(key); p++) {
        if (!memcmp(p, key, strlen(key)))
            break;
    }
    p += strlen(key);
    if (p >= end)
        return -1;

    for(; p<end; p++) {
        char *eptr;
        int64_t ret = strtol(p, &eptr, 10);
        if ((const uint8_t *)eptr != p)
            return ret;
    }
    return -1;
}

/**
 * Opens file and returns data in char array (string).
 * --------------------------------------------------
 * Note: Won't work for files > 4GB since fseek crashes.
 * However this won't be a problem since such big .xbm files
 * are too big to even show in the terminal window.
 */
uint8_t *read_file(uint8_t *fname) {
  FILE *fptr;
  uint8_t *data = 0;

  // Open stream if file exists
  if((fptr = fopen(fname, "r")) == NULL)
    return data;

  // Inspired by https://stackoverflow.com/a/174552
  fseek(fptr, 0, SEEK_END); // goto end of file
  long len = ftell(fptr); // get length
  fseek(fptr, 0, SEEK_SET); // goto start of file
  data = (uint8_t *)malloc(len); // allocate memory for the data
  if(data) {
    fread(data, 1, len, fptr); // copy data to buffer
  }
  fclose(fptr);
  printf("%s", data);
  return data;
}

/**
 * Decodes data and returns xmb pointer.
 * This is so that the xbm file does not need to be pre-compiled.
 */
XBM *decode_xbm(uint8_t *data) {
  XBM *xptr = (XBM *)malloc(sizeof(XBM)); // Allocates memory for XMB struct
  long len = strlen(data);
  const uint8_t *next, *ptr, *end, **cend;

  end = data + len;

  xptr->width = parse_str_int(data, len, "_width");
  xptr->height = parse_str_int(data, len, "_height");

  printf("width: %d, height: %d", xptr->width, xptr->height);

  next = memchr(data, '{', len); // get substring where data start
  printf("%s", next);

  if(!next)
    return NULL;

  ptr = next + 1;

  xptr->data = (uint8_t *)malloc(xptr->width * xptr->height);
  if(!data) {
    return NULL;
  }

  long i = 0;
  while(ptr < end) {
    while(*ptr != 'x') {
      ptr++;
    }
    uint8_t x = (uint8_t)strtol(ptr+1, NULL, 16);
    printf("%d\n", (int)x);
    printf("i:%d\n",i);
    // This causes segfault
    xptr->data[i++] = x;
    ptr++;
  }
  return xptr;
}

/**
 * Prints xmb picture to console by reading bytewise (the bits) line by line
 * and changing background color (ANSI) for every bit that is 'highlighted'.
 */
void print_xbm(XBM *p, uint8_t r, uint8_t g, uint8_t b) {

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
