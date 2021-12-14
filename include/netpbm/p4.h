#ifndef NETPBM_P4_H
#define NETPBM_P4_H

#include <stdint.h>

int parse_netpbm_p4(NETPBM *pbm, uint8_t *seek);
void print_netpbm_p4(NETPBM *p, uint8_t r, uint8_t g, uint8_t b);

#endif // NETPBM_P4_H
