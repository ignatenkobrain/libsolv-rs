#include <solv/poolarch.h>

unsigned char e_pool_arch2color(Pool *pool, Id arch) {
    return pool_arch2color(pool, arch);
}

int e_pool_colormatch(Pool *pool, Solvable *s1, Solvable *s2){
    return pool_colormatch(pool, s1, s2);
}
