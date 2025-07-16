
#include "../include/search.h"
#include <assert.h>
#include <float.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

void test_dijkstra() {
  int n = 4;
  long double graph[4][4] = {{0.0, 1.5, 0.0, 0.0},
                             {0.0, 0.0, 2.2, 0.0},
                             {0.0, 0.0, 0.0, 3.1},
                             {0.0, 0.0, 0.0, 0.0}};

  long double **ptr = malloc(4 * sizeof(long double *));
  assert(ptr);

  for (int i = 0; i < 4; i++) {
    ptr[i] = graph[i];
  }

  int start = 0;
  double expected[] = {0.0, 1.5, 3.7, 6.8};

  long double *distances = dijkstra(n, ptr, start);

  for (int i = 0; i < n; i++) {
    assert(fabsl(distances[i] - expected[i]) < 1e-6);
  }

  printf("Test passed!\n");
  free(distances);
}

int main() {
  test_dijkstra();

  return 0;
}
