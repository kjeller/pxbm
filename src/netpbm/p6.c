#include <stdlib.h>
#include <stddef.h>
#include <ctype.h>

#include "netpbm/netpbm.h"
#include "netpbm/p6.h"

int parse_netpbm_p6(NETPBM *pbm, uint8_t *seek)
{
    size_t i = 0;

    skip_comments_and_whitespace(&seek);
    pbm->max_value = atoi((char *) seek);
    while(isdigit(*seek)) ++seek;

    if (pbm->max_value > 255)
        pbm->bit_depth = 16;
    else
        pbm->bit_depth = 8;

    uint8_t *image_data = (uint8_t *) calloc(3 * pbm->width * pbm->height, pbm->bit_depth / 8);

    skip_comments_and_whitespace(&seek);
    while ((void *)seek - pbm->data < pbm->filesize)
    {
        image_data[i++] = *seek++;
    }

    free(pbm->data);
    pbm->data = (void *) image_data;
    return 0;
}
