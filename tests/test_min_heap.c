#include "../include/minheap.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void test_createMinHeap() {
  int cap = 10;
  min_heap *heap = createMinHeap(cap);

  if (heap && heap->size == 0 && heap->cap == cap && heap->arr && heap->pos)
    printf("test_createMinHeap: PASS\n");
  else
    printf("test_createMinHeap: FAIL\n");

  freeMinHeap(heap);
}

void test_isEmpty() {
  min_heap *heap = createMinHeap(5);
  if (isEmpty(heap))
    printf("test_isEmpty (initial): PASS\n");
  else
    printf("test_isEmpty (initial): FAIL\n");

  heap->arr[0] = malloc(sizeof(min_heap_n));
  heap->size = 1;
  if (!isEmpty(heap))
    printf("test_isEmpty (non-empty): PASS\n");
  else
    printf("test_isEmpty (non-empty): FAIL\n");

  freeMinHeap(heap);
}

void test_extractMin_and_insert_simulation() {
  min_heap *heap = createMinHeap(5);
  for (int i = 0; i < 5; i++) {
    heap->arr[i] = malloc(sizeof(min_heap_n));
    heap->arr[i]->v = i;
    heap->arr[i]->dist = 5 - i; // 5, 4, 3, 2, 1
    heap->pos[i] = i;
    heap->size++;
  }

  for (int i = (heap->size - 1) / 2; i >= 0; i--)
    minHeapify(heap, i);

  min_heap_n *min = extractMin(heap);
  if (min && min->dist == 1 && min->v == 4)
    printf("test_extractMin: PASS\n");
  else
    printf("test_extractMin: FAIL\n");

  freeMinHeap(heap);
}

void test_decKet() {
  min_heap *heap = createMinHeap(3);
  for (int i = 0; i < 3; i++) {
    heap->arr[i] = malloc(sizeof(min_heap_n));
    heap->arr[i]->v = i;
    heap->arr[i]->dist = 10.0 + i;
    heap->pos[i] = i;
    heap->size++;
  }

  decKey(heap, 2, 1.0);

  if (heap->arr[0]->v == 2 && heap->arr[0]->dist == 1.0)
    printf("test_decKey: PASS\n");
  else
    printf("test_decKey: FAIL\n");

  freeMinHeap(heap);
}

void test_isInMinHeap() {
  min_heap *heap = createMinHeap(5);
  for (int i = 0; i < 5; i++) {
    heap->arr[i] = malloc(sizeof(min_heap_n));
    heap->arr[i]->v = i;
    heap->arr[i]->dist = i;
    heap->pos[i] = i;
    heap->size++;
  }

  if (isInMinHeap(heap, 3))
    printf("test_isInMinHeap (existing): PASS\n");
  else
    printf("test_isInMinHeap (existing): FAIL\n");

  heap->size = 2;
  if (!isInMinHeap(heap, 4))
    printf("test_isInMinHeap (removed): PASS\n");
  else
    printf("test_isInMinHeap (removed): FAIL\n");

  freeMinHeap(heap);
}

int main() {
  test_createMinHeap();
  test_isEmpty();
  test_extractMin_and_insert_simulation();
  test_decKet();
  test_isInMinHeap();

  return 0;
}
