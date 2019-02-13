// Defines what features libsolvext was built with
#include <solv/solvversion.h>

// Always included
#include <solv/testcase.h>
#include <solv/solv_xfopen.h>
#include <solv/tools_util.h>

// Scraped from CMAKE
// TODO: Check we didn't miss anything

#if defined(LIBSOLVEXT_FEATURE_RPMDB) || defined(LIBSOLVEXT_FEATURE_RPMPKG)
#include <solv/pool_fileconflicts.h>
#include <solv/repo_rpmdb.h>
#endif

#ifdef LIBSOLVEXT_FEATURE_PUBKEY
#include <solv/repo_pubkey.h>
#endif

//TODO: cmake ENABLE_PGPVRFY


#ifdef LIBSOLVEXT_FEATURE_RPMMD
#include <solv/repo_repomdxml.h>
#include <solv/repo_rpmmd.h>
#include <solv/repo_deltainfoxml.h>
#include <solv/repo_updateinfoxml.h>
#endif

#ifdef LIBSOLVEXT_FEATURE_SUSEREPO
#include <solv/repo_content.h>
#include <solv/repo_products.h>
#include <solv/repo_releasefile_products.h>
#include <solv/repo_susetags.h>
#include <solv/repo_zyppdb.h>
#endif

#ifdef LIBSOLV_FEATURE_COMPLEX_DEPS
#if defined(LIBSOLVEXT_FEATURE_SUSEREPO) || defined(LIBSOLVEXT_FEATURE_RPMMD) || defined(LIBSOLVEXT_FEATURE_RPMDB) || defined(LIBSOLVEXT_FEATURE_RPMPKG)
#include <solv/pool_parserpmrichdep.h>
#endif
#endif

//TODO: cmake SUSE

#ifdef LIBSOLVEXT_FEATURE_COMPS
#include <solv/repo_comps.h>
#endif

#ifdef LIBSOLVEXT_FEATURE_DEBIAN
#include <solv/repo_deb.h>
#endif

#ifdef LIBSOLVEXT_FEATURE_HELIXREPO
#include <solv/repo_helix.h>
#endif

//TODO: cmake mdkrepo

#ifdef LIBSOLVEXT_FEATURE_ARCHREPO
#include <solv/repo_arch.h>
#endif

//TODO: cmake cudfrepo

#ifdef LIBSOLVEXT_FEATURE_HAIKU
#include <solv/repo_haiku.h>
#endif

#ifdef LIBSOLVEXT_FEATURE_APPDATA
#include <solv/repo_appdata.h>
#endif