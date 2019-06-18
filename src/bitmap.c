#include "bitmap.h"
#include <stdio.h>

void print_xmb(XMB *p, unsigned char r, unsigned char g, unsigned char b) {
    
    // Width of bitmap in bytes
 	int w_bytes; 
    w_bytes = p->width % 8 == 0 ? p->width / 8 : p->width / 8 + 1;

     for(int i = 0; i < p->height; i++) {
         for(int j = 0; j < w_bytes; j++) {
            // Get current byte with offset
            char byte = p->data[i * w_bytes + j];

            // Loop through the bits
            for(int k = 0; k < 8; k++) {
                if(byte & (1<<k))
                    printf("\033[48;2;%d;%d;%dm  ", r, g, b);
                else
                    printf("\033[48;2;0;0;0m  ");
            }
         }
		 printf("\n");
	}
	printf("\033[48;2;0;0;0m  ");
}