#include "../include/search.h"
#include "../include/minheap.h"
#include <float.h>
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

long double *dijkstra(int n, long double **g, int start) {
  long double *distances = (long double *)malloc(n * sizeof(long double));
  char *visited = (char *)calloc(n, sizeof(char));

  for (int i = 0; i < n; i++)
    distances[i] = DBL_MAX;
  distances[start] = 0;

  min_heap *heap = createMinHeap(n);
  for (int v = 0; v < n; v++) {
    insertMinHeap(heap, v, distances[v]);
  }

  while (!isEmpty(heap)) {
    min_heap_n *node = extractMin(heap);

    if (visited[node->v])
      continue;
    visited[node->v] = 1;

    if (node->dist > distances[node->v])
      continue;

    for (int v = 0; v < n; v++) {
      long double weight = g[node->v][v];
      if (weight > 0 && !visited[v]) {
        long double alt = distances[node->v] + weight;
        if (alt < distances[v]) {
          distances[v] = alt;
          decKey(heap, v, alt);
        }
      }
    }
  }

  free(visited);
  freeMinHeap(heap);

  return distances;
}

void greedy(int n, int start, graph_t g, int *path, int *visited) {
  for (int i = 0; i < n; i++)
    visited[i] = 0;

  int curr = 0;
  int current = start;

  path[curr++] = current;
  visited[current] = 1;

  for (int step = 1; step < n; step++) {
    int min = -69;
    int minIdx = -1;

    for (int i = 0; i < n; i++) {
      if (!visited[i] && g[current][i] > 0) {
        if (min == -69 || g[current][i] < min) {
          min = g[current][i];
          minIdx = i;
        }
      }
    }

    if (minIdx == -1)
      break;

    current = minIdx;
    path[curr++] = current;
    visited[current] = 1;
  }

  if (g[current][start] > 0) {
    path[curr++] = start;
  } else {
    // idk do smth mb
  }

  path[curr] = -1;
}
