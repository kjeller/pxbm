#ifndef NETPBM_H
#define NETPBM_H

#include <stdint.h>

typedef struct _netpbm
{
    uint8_t type;
    uint32_t filesize;
    uint32_t width;
    uint32_t height;
    uint8_t bit_depth;
    uint32_t max_value;
    void *data;

} NETPBM;

int parse_netpbm(NETPBM *pbm);
void print_netpbm(NETPBM *p, uint8_t r, uint8_t g, uint8_t b);
NETPBM *read_pbm_file(char *fname);

void skip_comments_and_whitespace(uint8_t **seek_ptr);

#endif // NETPBM_H
