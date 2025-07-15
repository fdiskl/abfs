#include "../include/search.h"
#include <stdio.h>

void dfs(int n, int node, graph_t g, int *visited) {
  visited[node] = 1;

  printf("visited %d \n", node);

  for (int i = 0; i < n; i++)
    if (g[node][i] != 0.0 && !visited[i])
      dfs(n, i, g, visited);
}

void bfs(graph_t g) {}
