#include "../include/gen.h"
#include "../include/search.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int count_lines(FILE *fp) {
  int count = 0;
  char buffer[128];
  while (fgets(buffer, sizeof(buffer), fp)) {
    count++;
  }
  rewind(fp);
  return count;
}

void read_visited(FILE *fp, int *visited) {
  int node;
  while (fscanf(fp, "visited %d", &node) == 1) {
    visited[node] = 1;
  }
  rewind(fp);
}

int main() {
  graph_t g;
  int n = 10000;
  gen_graph(n, &g);

  int *visited_dfs = calloc(n, sizeof(int));
  int *visited_bfs = calloc(n, sizeof(int));
  if (!visited_dfs || !visited_bfs)
    return 1;

  FILE *dfs_fp = tmpfile();
  dfs(n, 0, g, visited_dfs, dfs_fp);

  memset(visited_bfs, 0, n * sizeof(int));
  FILE *bfs_fp = tmpfile();
  bfs(n, 0, g, visited_bfs, bfs_fp);

  rewind(dfs_fp);
  rewind(bfs_fp);

  int dfs_lines, bfs_lines;
  printf("DFS visited: %d nodes\n", dfs_lines = count_lines(dfs_fp));
  printf("BFS visited: %d nodes\n", bfs_lines = count_lines(bfs_fp));

  if (dfs_lines != bfs_lines || dfs_lines != n) {
    printf("Invalid amount of nodes was visited\n");
    return -1;
  }

  memset(visited_dfs, 0, n * sizeof(int));
  memset(visited_bfs, 0, n * sizeof(int));
  read_visited(dfs_fp, visited_dfs);
  read_visited(bfs_fp, visited_bfs);

  int mismatch = 0;
  for (int i = 0; i < n; i++) {
    if (visited_dfs[i] != visited_bfs[i]) {
      printf("Mismatch at node %d\n", i);
      mismatch = 1;
    }
  }

  if (!mismatch)
    printf(":)  DFS and BFS visited the same set of nodes\n");
  else
    printf(">:( DFS and BFS did not visit the same nodes\n");

  free(visited_dfs);
  free(visited_bfs);
  for (int i = 0; i < n; i++)
    free(g[i]);
  free(g);
  fclose(dfs_fp);
  fclose(bfs_fp);

  return 0;
}
