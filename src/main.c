#include "pxbm.h"
#include "netpbm/netpbm.h"

#include <stdio.h>
#include <stdlib.h>
#include <getopt.h>

typedef enum _format
{
    FORMAT_NETPBM,
    FORMAT_XBM,
} FORMAT;

static FORMAT format = FORMAT_XBM;

static const struct option long_options[] =
{
    {"help", no_argument, NULL, 'h'},
    {"netpbm", no_argument, NULL, 'n'},
    {"xbm", no_argument, NULL, 'x'},
    {0, 0, 0, 0}
};

void usage(int argc, char *argv[])
{
    fprintf(stderr,
            "Usage:\n"
            "\t%s [options] <filepath> [red] [green] [blue]\n"
            "Options:\n"
            "\t-x, --xbm    Print an xbm bitmap (default)\n"
            "\t-n, --netpbm Print a netpbm bitmap\n"
            "\t-h, --help   Show this help\n"
            "Colors:\n"
            "\tSupply integer arguments in the range 0-255 for\n"
            "\tthe red, green, and blue channels. Only applies to\n"
            "\tbitmaps that do not support color.\n"
            ,
            argv[0]);
}

void netpbm(char *filename, int r, int g, int b)
{
    NETPBM *pbm = read_pbm_file(filename);

    if (pbm)
    {
        int res = parse_netpbm(pbm);

        if(res != 0)
        {
            fprintf(stderr, "Error: Could not parse netpbm data\n");
            exit(EXIT_FAILURE);
        }

        print_netpbm(pbm, r, g, b);
    }

    else
    {
        fprintf(stderr, "Error: Could not read file. Corrupted or missing?\n");
        exit(EXIT_FAILURE);
    }
}

void xbm(char *filename, int r, int g, int b)
{
    char *data = read_xbm_file(filename);

    if(data)
    {
        XBM xptr;
        int res = parse_xbm(&xptr, data);

        if(res != 0)
        {
            fprintf(stderr, "Error: Could not parse xbm data\n");
            exit(EXIT_FAILURE);
        }
        // Handle RGB input
        print_xbm(&xptr, r, g, b);
    }

    else
    {
        fprintf(stderr, "Error: Could not read file. Corrupted or missing?\n");
        exit(EXIT_FAILURE);
    }
}

int main(int argc, char *argv[])
{
    int c;
    while (1)
    {
        int option_index = 0;
        c = getopt_long(argc, argv, "hxn", long_options, &option_index);
        if (c == -1)
            break;

        switch (c)
        {
            case 'h':
                usage(argc, argv);
                exit(EXIT_SUCCESS);
                break;

            case 'x':
                format = FORMAT_XBM;
                break;

            case 'n':
                format = FORMAT_NETPBM;
                break;

            default:
                usage(argc, argv);
                exit(EXIT_FAILURE);
                break;
        }
    }

    if(optind == argc)
    {
        fprintf(stderr, "Error: Missing filename\n");
        usage(argc, argv);
        return(1);
    }

    char *filename = argv[optind++];

    int r = 0;
    int g = 0;
    int b = 0;
    if (optind < argc)
        r = atoi(argv[optind++]);
    if (optind < argc)
        g = atoi(argv[optind++]);
    if (optind < argc)
        b = atoi(argv[optind++]);

    switch (format)
    {
        case FORMAT_XBM:
            xbm(filename, r, g, b);
            break;
        case FORMAT_NETPBM:
            netpbm(filename, r, g, b);
            break;
        default:
            fprintf(stderr, "Unknown format\n");
            exit(EXIT_FAILURE);
            break;
    }
    return EXIT_SUCCESS;
}
