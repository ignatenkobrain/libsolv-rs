#include <solv/queue.h>

// Queue
void
queue_empty_static(Queue *q)
{
  queue_empty(q);
}
Id
queue_shift_static(Queue *q)
{
  return queue_shift(q);
}

Id
queue_pop_static(Queue *q)
{
  return queue_pop(q);
}
void
queue_unshift_static(Queue *q, Id id)
{
  queue_unshift(q, id);
}
void
queue_push_static(Queue *q, Id id)
{
  queue_push(q, id);
}
void
queue_pushunique_static(Queue *q, Id id)
{
  queue_pushunique(q, id);
}
void
queue_push2_static(Queue *q, Id id1, Id id2)
{
  queue_push2(q, id1, id2);
}

void
queue_truncate_static(Queue *q, int n)
{
  queue_truncate(q, n);
}
