typedef unsigned char uint8_t;
typedef int int64_t;

typedef struct _xbm {
    uint8_t *data;
    int width;
    int height;
} XBM;

uint8_t *read_file(uint8_t *fname);
XBM *decode_xbm(uint8_t *data);
void print_xbm(XBM *p, uint8_t r, uint8_t g, uint8_t b);
