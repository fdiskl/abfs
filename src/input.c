#include "../include/input.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int read(graph_t *graph, FILE *f) {

  int n;
  if (fscanf(f, "%d", &n) != 1) {
    printf("Can't read n\n");
    return -1;
  }

  if (!(*graph = (double long **)malloc(n * sizeof(double long *)))) {
    printf("Can't alloc graph matrix\n");
    return -1;
  }

  for (int i = 0; i < n; i++) {
    (*graph)[i] = (long double *)malloc(n * sizeof(long double));
    if (!(*graph)[i]) {
      printf("Can't alloc graph matrix row %d\n", i);
      return -1;
    }

    if (fscanf(f, "%Lf", &((*graph)[i][0])) != 1) {
      printf("Can't read graph matrix value %d %d\n", i, 0);
      return -1;
    }

    for (int j = 1; j < n; j++) {
      if (fscanf(f, ", %Lf", &((*graph)[i][j])) != 1) {
        printf("Can't read graph matrix value %d %d\n", i, j);
        return -1;
      }
    }
  }

  return n;
}

int read_answer(FILE *f, int **path) {

  int *nums = NULL;
  size_t size = 0;
  char buffer[64];

  while (fscanf(f, "%63s", buffer) == 1) {
    if (strcmp(buffer, "EOF") == 0)
      break;

    int val;
    if (sscanf(buffer, "%d", &val) != 1) {
      fprintf(stderr, "Invalid number: %s\n", buffer);
      continue;
    }

    val--;

    nums = realloc(nums, (size + 1) * sizeof(long double));
    if (!nums) {
      fprintf(stderr, "Memory allocation failed\n");
      fclose(f);
      return 1;
    }
    nums[size++] = val;
  }

  *path = nums;

  return size;
}
