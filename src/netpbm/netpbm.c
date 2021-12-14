#include <stdio.h>
#include <stdlib.h>
#include <ctype.h>

#include "util.h"

#include "netpbm/netpbm.h"

#include "netpbm/p1.h"
#include "netpbm/p4.h"

NETPBM *read_pbm_file(char *fname)
{
    size_t filesize;
    uint8_t *data = read_file(fname, &filesize);

    if (data == NULL)
        return NULL;

    NETPBM *pbm = calloc(1, sizeof(NETPBM));
    pbm->filesize = filesize;
    pbm->data = data;
    return pbm;
}

void print_netpbm(NETPBM *p, uint8_t r, uint8_t g, uint8_t b)
{
    switch (p->type)
    {
        case '1':
            print_netpbm_p1(p, r, g, b);
            break;

        case '4':
            print_netpbm_p4(p, r, g, b);
            break;

        default:
            break;
    }
}

int parse_netpbm(NETPBM *pbm)
{
    pbm->type = ((uint8_t *) pbm->data)[1];

    uint8_t *seek = pbm->data + 2;
    skip_comments_and_whitespace(&seek);
    pbm->width = atoi((char *) seek);
    while(isdigit(*seek)) ++seek;

    skip_comments_and_whitespace(&seek);
    pbm->height = atoi((char *) seek);
    while(isdigit(*seek)) ++seek;

    switch (pbm->type)
    {
        case '1':
            return parse_netpbm_p1(pbm, seek);
        case '4':
            return parse_netpbm_p4(pbm, seek);
        default:
            break;
    }
    return -1;
}

void skip_comments_and_whitespace(uint8_t **seek_ptr)
{
    while (isspace(**seek_ptr) || **seek_ptr == '#')
    {
        if (**seek_ptr == '#')
            while (**seek_ptr != '\n') *seek_ptr += 1;
        while (isspace(**seek_ptr)) *seek_ptr += 1;
    }
}

