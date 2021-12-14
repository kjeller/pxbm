#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include "util.h"

uint8_t *read_file(char *fname, size_t *filesize)
{
    FILE *fptr;
    uint8_t *data = 0;

    // Open stream if file exists
    if((fptr = fopen(fname, "r")) == NULL)
        return data;

    // Inspired by https://stackoverflow.com/a/174552
    fseek(fptr, 0, SEEK_END); // goto end of file
    size_t len = ftell(fptr); // get length
    fseek(fptr, 0, SEEK_SET); // goto start of file
    data = (uint8_t *) malloc(len); // allocate memory for the data
    if(data) {
        fread(data, 1, len, fptr); // copy data to buffer
    }
    fclose(fptr);

    if (filesize)
        *filesize = len;

    return data;
}

