#ifndef MIN_HEAP_H
#define MIN_HEAP_H

typedef struct min_heap min_heap;
typedef struct min_heap_n min_heap_n;

struct min_heap_n {
  int v;
  long double dist;
};

struct min_heap {
  int size;
  int cap;
  int *pos; // vertex -> index

  min_heap_n **arr;
};

min_heap *createMinHeap(int capacity);
void freeMinHeap(min_heap *heap);
int isEmpty(min_heap *heap);
min_heap_n *extractMin(min_heap *heap);
void decKey(min_heap *heap, int vertex, long double dist);
int isInMinHeap(min_heap *heap, int vertex);
void minHeapify(min_heap *h, int i);
void insertMinHeap(min_heap *h, int v, long double dist);

#endif
