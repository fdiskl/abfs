#include "../include/gen.h"
#include <stdlib.h>
#include <time.h>

// 1/2 chance
#define CHANCE 2

#define MAX_W 10

void gen_graph(int n, graph_t *g) {
  *g = (long double **)malloc(n * sizeof(long double *));
  for (int i = 0; i < n; i++) {
    (*g)[i] = (long double *)calloc(n, sizeof(long double));
  }

  srand((unsigned int)time(NULL));

  for (int i = 0; i < n; i++) {
    for (int j = i + 1; j < n; j++) {
      if (rand() % CHANCE) {
        double weight = (rand() % MAX_W) + 1;
        (*g)[i][j] = weight;
        (*g)[j][i] = weight; // symmetry
      }
    }
  }
}
