#ifndef NETPBM_P1_H
#define NETPBM_P1_H

#include <stdint.h>

int parse_netpbm_p1(NETPBM *pbm, uint8_t *seek);
void print_netpbm_p1(NETPBM *p, uint8_t r, uint8_t g, uint8_t b);

#endif // NETPBM_P1_H

