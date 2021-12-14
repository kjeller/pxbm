#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>
#include <arpa/inet.h>

#include "netpbm/netpbm.h"
#include "netpbm/ppm.h"

void print_netppm(NETPBM *p)
{
    uint8_t *data = (uint8_t *) p->data;
    for (size_t i = 0; i < p->height; i++)
    {
        for (size_t j = 0; j < p->width * 3; j += 3)
        {
            size_t r = 0;
            size_t g = 0;
            size_t b = 0;

            if (p->bit_depth == 16)
            {
                // clamping to 0-255, but still supporting 0-65535
                r = ntohs(((uint16_t *) data)[i * 3*p->width + j + 0]) * 65535 / p->max_value / 255;
                g = ntohs(((uint16_t *) data)[i * 3*p->width + j + 1]) * 65535 / p->max_value / 255;
                b = ntohs(((uint16_t *) data)[i * 3*p->width + j + 2]) * 65535 / p->max_value / 255;
            }
            else
            {
                r = (size_t) data[i * 3*p->width + j + 0] * 255 / p->max_value;
                g = (size_t) data[i * 3*p->width + j + 1] * 255 / p->max_value;
                b = (size_t) data[i * 3*p->width + j + 2] * 255 / p->max_value;
            }

            printf("\033[48;2;%lu;%lu;%lum  \033[m", r, g, b);
        }
        puts("");
    }
}

