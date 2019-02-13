#include <solv/repo.h>

const char *e_repo_name(const Repo *repo) {
    return repo_name(repo);
}
Repo *e_pool_id2repo(Pool *pool, Id repoid) {
    return pool_id2repo(pool, repoid);
}
int e_pool_disabled_solvable(const Pool *pool, Solvable *s) {
    return pool_disabled_solvable(pool, s);
}
int e_pool_installable(const Pool *pool, Solvable *s) {
    return pool_installable(pool, s);
}
