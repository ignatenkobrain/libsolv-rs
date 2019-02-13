#include <solv/dirpool.h>

Id
e_dirpool_parent(Dirpool *dp, Id did) {
    return dirpool_parent(dp, did);
}

Id
e_dirpool_sibling(Dirpool *dp, Id did) {
    return dirpool_sibling(dp, did);
}

Id
e_dirpool_child(Dirpool *dp, Id did) {
    return dirpool_child(dp, did);
}

void
e_dirpool_free_dirtraverse(Dirpool *dp) {
    return dirpool_free_dirtraverse(dp);
}

Id
e_dirpool_compid(Dirpool *dp, Id did) {
    return dirpool_compid(dp, did);
}
