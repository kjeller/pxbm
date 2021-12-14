#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>
#include <arpa/inet.h>

#include "netpbm/netpbm.h"
#include "netpbm/pgm.h"

void print_netpgm(NETPBM *p)
{
    uint8_t *data = (uint8_t *) p->data;
    for (size_t i = 0; i < p->height; i++)
    {
        for (size_t j = 0; j < p->width; j++)
        {
            size_t pix = 0;

            if (p->bit_depth == 16)
            {
                // clamping to 0-255, but still supporting 0-65535
                pix = ntohs(((uint16_t *) data)[i * p->width + j]) * 65535 / p->max_value / 255;
            }
            else
                pix = data[i * p->width + j] * 255 / p->max_value;

            printf("\033[48;2;%lu;%lu;%lum  \033[m", pix, pix, pix);
        }
        puts("");
    }
}
