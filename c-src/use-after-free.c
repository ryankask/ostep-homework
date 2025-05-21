#include <stdio.h>
#include <stdlib.h>

/* Create a program that allocates an array of integers (as above), frees */
/* them, and then tries to print the value of one of the elements of */
/* the array. Does the program run? What happens when you use */
/* valgrind on it? */

int main() {
  int *x = (int *)malloc(sizeof(int) * 100);
  x[50] = 5080;
  free(x);
  printf("x[50]=%d\n", x[50]);
  return 0;
}
