#include <solv/queue.h>

// Queue
static inline void
queue_empty_real(Queue *q)
{
  queue_empty(q);
}
static inline Id
queue_shift_real(Queue *q)
{
  return queue_shift(q);
}
static inline Id
queue_pop_real(Queue *q)
{
  return queue_pop(q);
}
static inline void
queue_unshift_real(Queue *q, Id id)
{
  queue_unshift(q, id);
}
static inline void
queue_push_real(Queue *q, Id id)
{
  queue_push(q, id);
}
static inline void
queue_pushunique_real(Queue *q, Id id)
{
  queue_pushunique(q, id);
}
static inline void
queue_push2_real(Queue *q, Id id1, Id id2)
{
  queue_push2(q, id1, id2);
}

static inline void
queue_truncate_real(Queue *q, int n)
{
  queue_truncate(q, n);
}
