#include "../include/input.h"
#include <stdio.h>
#include <stdlib.h>

int read(graph_t *graph, FILE *f) {

  int n;
  if (fscanf(f, "%d", &n) != 1) {
    printf("Can't read n\n");
    return -1;
  }

  if (!(*graph = (double long **)malloc(n * sizeof(double long *)))) {
    printf("Can't alloc graph matrix\n");
    return -1;
  }

  for (int i = 0; i < n; i++) {
    (*graph)[i] = (long double *)malloc(n * sizeof(long double));
    if (!(*graph)[i]) {
      printf("Can't alloc graph matrix row %d\n", i);
      return -1;
    }

    for (int j = 0; j < n; j++) {
      if (fscanf(f, "%Lf", &((*graph)[i][j])) != 1) {
        printf("Can't read graph matrix value %d %d\n", i, j);
        return -1;
      }
    }
  }

  return n;
}
