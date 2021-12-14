#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>

#include "netpbm/netpbm.h"
#include "netpbm/p4.h"

int parse_netpbm_p4(NETPBM *pbm, uint8_t *seek)
{
    size_t i = 0;
    uint8_t *image_data = (uint8_t *) calloc(pbm->width * pbm->height, 1);

    skip_comments_and_whitespace(&seek);
    while ((void *)seek - pbm->data < pbm->filesize)
    {
        image_data[i++] = *seek++;
    }

    free(pbm->data);
    pbm->bit_depth = 1;
    pbm->max_value = 1;
    pbm->data = (void *) image_data;
    return 0;
}

void print_netpbm_p4(NETPBM *p, uint8_t r, uint8_t g, uint8_t b)
{
    uint8_t *data = (uint8_t *) p->data;
    size_t width_in_bytes = (p->width + 7) / 8;
    for (size_t i = 0; i < p->height; i++)
    {
        for (size_t j = 0; j < width_in_bytes; j++)
        {
            uint8_t byte = data[i * width_in_bytes + j];

            for(int k = 7; k >= 0; k--)
            {
                if(byte & (1 << k))
                    printf("\033[48;2;%u;%u;%um  \033[0m", r, g, b);
                else
                    printf("  ");
            }

        }
        puts("");
    }
}
