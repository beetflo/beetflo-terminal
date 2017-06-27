
#include <stdio.h>
/* #include <gtk/gtk.h> */

extern void beets_init() {
  printf("Hello, from the bridge!!\n");
}

extern void beets_init_gtk() {
  printf("Aye aye captain!!\n");
}
