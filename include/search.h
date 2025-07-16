#ifndef SEARCH_H
#define SEARCH_H

#include "graph.h"
#include <stdio.h>
#include <stdlib.h>

// g should be NxN
// visited should be N
void dfs(int n, int node, graph_t g, int *visited, FILE *f);

// g should be NxN
// visited should be N
void bfs(int n, int node, graph_t g, int *visited, FILE *f);

// g should be NxN
// returns array of len N with distances from start to node i
long double *dijkstra(int n, graph_t g, int start);

static inline void run_dfs(int n, int start, graph_t g) {
  int *visited = (int *)calloc(n, sizeof(int));
  if (visited) {
    dfs(n, start, g, visited, stdout);
    free(visited);
  }
}

static inline void run_bfs(int n, int start, graph_t g) {
  int *visited = (int *)calloc(n, sizeof(int));
  if (visited) {
    bfs(n, start, g, visited, stdout);
    free(visited);
  }
}

#endif
