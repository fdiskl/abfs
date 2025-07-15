#include "../include/search.h"
#include <stdio.h>
#include <stdlib.h>

void dfs(int n, int node, graph_t g, int *visited, FILE *f) {
  visited[node] = 1;

  fprintf(f, "visited %d\n", node);

  for (int i = 0; i < n; i++)
    if (g[node][i] != 0.0 && !visited[i])
      dfs(n, i, g, visited, f);
}

void bfs(int n, int start, graph_t g, int *visited, FILE *f) {
  int *queue = malloc(n * sizeof(int));
  int front = 0, rear = 0;

  visited[start] = 1;
  queue[rear++] = start;

  while (front < rear) {
    int node = queue[front++];
    fprintf(f, "visited %d\n", node);

    for (int i = 0; i < n; i++) {
      if (g[node][i] != 0.0 && !visited[i]) {
        visited[i] = 1;
        queue[rear++] = i;
      }
    }
  }

  free(queue);
}
