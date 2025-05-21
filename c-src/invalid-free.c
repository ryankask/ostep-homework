#include <stdio.h>
#include <stdlib.h>

/* Now pass a funny value to free (e.g., a pointer in the middle of the */
/* array you allocated above). What happens? Do you need tools to */
/* find this type of problem? */

int main() {
  int *x = (int *)malloc(sizeof(int) * 100);
  free(&x[24]);
  return 0;
}
