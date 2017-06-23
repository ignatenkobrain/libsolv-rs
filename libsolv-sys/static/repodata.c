#include <solv/repodata.h>

Repokey *
e_repodata_id2key(Repodata *data, Id keyid) {
    return repodata_id2key(data, keyid);
}

Id *
e_repodata_id2schema(Repodata *data, Id schemaid) {
    return repodata_id2schema(data, schemaid);
}

int
e_repodata_precheck_keyname(Repodata *data, Id keyname) {
    return repodata_precheck_keyname(data, keyname);
}

int
e_repodata_has_keyname(Repodata *data, Id keyname) {
    return repodata_has_keyname(data, keyname);
}
