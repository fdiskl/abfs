#include "../include/input.h"
#include "../include/search.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define FILE_PATH "input.txt"

int main(void) {
  FILE *f = fopen(FILE_PATH, "r");

  if (!f) {
    printf("NO FILE GO CRY ABOUT IT\n");
    return 1;
  }

  graph_t graph;
  int n;
  if ((n = read(&graph, f)) == -1) {
    return 1;
  }

  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n; j++) {
      printf("%Lf ", graph[i][j]);
    }
    printf("\n");
  }

  printf("--DFS--\n");
  run_dfs(n, 0, graph);
  printf("--BFS--\n");
  run_bfs(n, 0, graph);
}
