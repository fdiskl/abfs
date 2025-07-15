#ifndef SEARCH_H
#define SEARCH_H

#include "graph.h"

// g should be NxN
// visited should be N
void dfs(int n, int node, graph_t g, int *visited);
void bfs(int n, int start, graph_t g, int *visited);

#endif
