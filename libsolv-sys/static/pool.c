#include <solv/pool.h>

Solvable *e_pool_id2solvable(const Pool *pool, Id p) {
    return pool_id2solvable(pool, p);
}
const char *e_pool_solvid2str(Pool *pool, Id p) {
    return pool_solvid2str(pool, p);
}
int e_pool_match_nevr(Pool *pool, Solvable *s, Id d) {
    return pool_match_nevr(pool, s, d);
}
Id e_pool_whatprovides(Pool *pool, Id d) {
    return pool_whatprovides(pool, d);
}
Id *e_pool_whatprovides_ptr(Pool *pool, Id d) {
    return pool_whatprovides_ptr(pool, d);
}
