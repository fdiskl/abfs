#ifndef INPUT_H
#define INPUT_H

#include "graph.h"
#include "stdio.h"

int read(graph_t *graph, FILE *f);

// return path n*2
int read_answer(FILE *f, int **arr);

#endif
