#include "../include/input.h"
#include "../include/search.h"
#include <stdio.h>

#define FILE_PATH "./berlin52.txt"
#define ANSWER_FILE_PATH "./berlin52_answer.txt"

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

  fclose(f);

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
  printf("--DIJKSTRA--\n");
  long double *distances = dijkstra(n, graph, 0);
  for (int i = 0; i < n; i++)
    printf("%Lf ", distances[i]);
  printf("\n");
  printf("--GREEDY--\n");
  run_greedy(n, graph);

  FILE *ff = fopen(ANSWER_FILE_PATH, "r");
  if (!ff) {
    printf("NO ANSWER\n");
  }
  int *arr;
  int k = read_answer(ff, &arr);
  printf("ANSWER (%d)\n", k);
  calculate_path_len(k, n, arr, graph);
}
