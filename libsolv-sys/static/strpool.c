#include <solv/strpool.h>

const char *
e_stringpool_id2str(Stringpool *ss, Id id) {
    return stringpool_id2str(ss, id);
}