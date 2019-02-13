#include <solv/bitmap.h>

void e_map_empty(Map *m)
{
  map_empty(m);
}
void e_map_set(Map *m, int n)
{
  map_set(m, n);
}
void e_map_setall(Map *m)
{
  map_setall(m);
}
void e_map_clr(Map *m, int n)
{
  map_clr(m, n);
}
int e_map_tst(Map *m, int n)
{
  return map_tst(m, n);
}
