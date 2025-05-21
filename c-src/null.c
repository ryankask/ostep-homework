#include <stdio.h>

/* First, write a simple program called null.c that creates a pointer */
/* to an integer, sets it to NULL, and then tries to dereference it. Com- */
/* pile this into an executable called null. What happens when you */
/* run this program? */

int main() {
  int *x = NULL;
  printf("value: %d", *x);
  return 0;
}
