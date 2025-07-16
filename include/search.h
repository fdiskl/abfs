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

// path should be of len n
void greedy(int n, int start, graph_t g, int *path, int *visited);

static inline void calculate_path_len(int m, int n, int *path, graph_t g) {
  long double res = 0.0;

  for (int i = 0; i < m; i++) {
    int u = path[i];
    int v = path[i + 1];

    if (u == -1 || v == -1)
      break;

    res += g[u][v];
  }

  printf("LEN: %.12Lf\n", res);
}

static inline void run_greedy(int n, graph_t g) {
  int *path = (int *)calloc(n * 2, sizeof(int));
  int *visited = (int *)calloc(n * 2, sizeof(int));
  if (path) {
    greedy(n, 0, g, path, visited);

    for (int i = 0; i < n * 2; i++) {
      if (path[i] == -1)
        break;
      printf("%d\n", path[i]);
    }
    printf("\n");
  }
  calculate_path_len(n * 2, n, path, g);
}

#endif
