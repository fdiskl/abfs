#ifndef SEARCH_H
#define SEARCH_H

#include "graph.h"

// g should be NxN
// visited should be N
void dfs(int n, int node, graph_t g, int *visited);
void bfs(graph_t g);

#endif
