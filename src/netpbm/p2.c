#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>
#include <arpa/inet.h>

#include "netpbm/netpbm.h"
#include "netpbm/p2.h"

int parse_netpbm_p2(NETPBM *pbm, uint8_t *seek)
{
    size_t i = 0;

    skip_comments_and_whitespace(&seek);
    pbm->max_value = atoi((char *) seek);
    while(isdigit(*seek)) ++seek;

    if (pbm->max_value > 255)
        pbm->bit_depth = 16;
    else
        pbm->bit_depth = 8;

    uint8_t *image_data = (uint8_t *) calloc(pbm->width * pbm->height, pbm->bit_depth / 8);

    skip_comments_and_whitespace(&seek);
    while ((void *)seek - pbm->data < pbm->filesize)
    {
        if (pbm->bit_depth == 16)
        {
            ((uint16_t *) image_data)[i] = htons(atoi((char *) seek));
        }
        else
        {
            image_data[i] = atoi((char *) seek);
        }
        while(isdigit(*seek)) ++seek;
        skip_comments_and_whitespace(&seek);
        i++;
    }

    free(pbm->data);
    pbm->data = (void *) image_data;
    return 0;
}
