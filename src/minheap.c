#include "../include/minheap.h"
#include <stdlib.h>

// TODO: arena allocator

static min_heap_n *newMinHeapNode(int v, long double dist) {
  min_heap_n *node = (min_heap_n *)malloc(sizeof(min_heap_n));
  node->v = v;
  node->dist = dist;

  return node;
}

min_heap *createMinHeap(int cap) {
  min_heap *h = (min_heap *)malloc(sizeof(min_heap));
  h->pos = (int *)malloc(cap * sizeof(int));
  h->size = 0;
  h->cap = cap;
  h->arr = (min_heap_n **)malloc(cap * sizeof(min_heap_n *));
  return h;
}

static void swapMinHeapNode(min_heap_n **a, min_heap_n **b) {
  min_heap_n *t = *a;
  *a = *b;
  *b = t;
}

inline int isEmpty(min_heap *h) { return h->size == 0; }

void minHeapify(min_heap *h, int i) {
  int small = i;
  int l = 2 * i + 1;
  int r = l + 1;

  if (l < h->size && h->arr[l]->dist < h->arr[small]->dist)
    small = l;
  if (r < h->size && h->arr[r]->dist < h->arr[small]->dist)
    small = r;

  if (small != i) {
    min_heap_n *sn = h->arr[small];
    min_heap_n *curr = h->arr[i];

    h->pos[sn->v] = i;
    h->pos[curr->v] = small;

    swapMinHeapNode(&h->arr[small], &h->arr[i]);
    minHeapify(h, small);
  }
}

void freeMinHeap(min_heap *heap) {
  for (int i = 0; i < heap->size; i++)
    free(heap->arr[i]);

  free(heap->arr);
  free(heap->pos);
  free(heap);
}

min_heap_n *extractMin(min_heap *h) {
  if (isEmpty(h))
    return NULL;

  min_heap_n *root = h->arr[0];
  min_heap_n *last = h->arr[h->size - 1];
  h->arr[0] = last;

  h->pos[root->v] = --h->size;
  h->pos[last->v] = 0;

  minHeapify(h, 0);

  return root;
}

void decKey(min_heap *h, int v, long double dist) {
  int i = h->pos[v];
  h->arr[i]->dist = dist;

  while (i && h->arr[i]->dist < h->arr[(i - 1) / 2]->dist) {
    int parent = (i - 1) / 2;
    h->pos[h->arr[i]->v] = parent;
    h->pos[h->arr[parent]->v] = i;

    swapMinHeapNode(&h->arr[i], &h->arr[parent]);
    i = parent;
  }
}

int isInMinHeap(min_heap *h, int v) { return h->pos[v] < h->size; }

void insertMinHeap(min_heap *h, int v, long double dist) {
  if (h->size == h->cap)
    return;

  int i = h->size;
  h->arr[i] = newMinHeapNode(v, dist);
  h->pos[v] = i;
  h->size++;

  while (i && h->arr[i]->dist < h->arr[(i - 1) / 2]->dist) {
    int parent = (i - 1) / 2;

    h->pos[h->arr[i]->v] = parent;
    h->pos[h->arr[parent]->v] = i;

    swapMinHeapNode(&h->arr[i], &h->arr[parent]);
    i = parent;
  }
}
