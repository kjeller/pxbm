#include <stdlib.h>

typedef struct _xbm {
    char *data;
    int width;
    int height;
} XBM;

char *read_xbm_file(char *fname);
int parse_xbm(XBM *xptr, char *data);
void print_xbm(XBM *p, unsigned char r, unsigned char g, unsigned char b);
