typedef struct _xmb {
    unsigned char *data;
    int width;
    int height;
} XMB;

void print_xmb(XMB *p, unsigned char r, unsigned char g, unsigned char b);