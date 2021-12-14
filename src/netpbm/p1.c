#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>

#include "netpbm/netpbm.h"
#include "netpbm/p1.h"

int parse_netpbm_p1(NETPBM *pbm, uint8_t *seek)
{
    size_t i = 0;
    uint8_t *image_data = (uint8_t *) calloc(pbm->width * pbm->height, 1);

    skip_comments_and_whitespace(&seek);
    while ((void *)seek - pbm->data < pbm->filesize)
    {
        image_data[i] = atoi((char *) seek);
        while(isdigit(*seek)) ++seek;
        skip_comments_and_whitespace(&seek);
        i++;
    }

    free(pbm->data);
    pbm->bit_depth = 1;
    pbm->max_value = 1;
    pbm->data = (void *) image_data;
    return 0;
}

void print_netpbm_p1(NETPBM *p, uint8_t r, uint8_t g, uint8_t b)
{
    uint8_t *data = (uint8_t *) p->data;
    for (size_t i = 0; i < p->height; i++)
    {
        for (size_t j = 0; j < p->width; j++)
        {
            uint8_t pix = data[i * p->width + j];
            if (pix)
                printf("\033[48;2;%u;%u;%um  \033[m", r, g, b);
            else
                printf("  ");
        }
        puts("");
    }
}

