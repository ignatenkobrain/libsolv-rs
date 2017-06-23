#include <solv/queue.h>

void
e_queue_empty(Queue *q)
{
  queue_empty(q);
}
Id
e_queue_shift(Queue *q)
{
  return queue_shift(q);
}
Id
e_queue_pop(Queue *q)
{
  return queue_pop(q);
}
void
e_queue_unshift(Queue *q, Id id)
{
  queue_unshift(q, id);
}
void
e_queue_push(Queue *q, Id id)
{
  queue_push(q, id);
}
void
e_queue_pushunique(Queue *q, Id id)
{
  queue_pushunique(q, id);
}
void
e_queue_push2(Queue *q, Id id1, Id id2)
{
  queue_push2(q, id1, id2);
}
void
e_queue_truncate(Queue *q, int n)
{
  queue_truncate(q, n);
}
