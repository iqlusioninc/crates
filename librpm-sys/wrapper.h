// Wrapper for librpm headers to be passed to bindgen

#include <rpm/header.h>
#include <rpm/rpmdb.h>
#include <rpm/rpmds.h>
#include <rpm/rpmio.h>
#include <rpm/rpmlog.h>
#include <rpm/rpmpol.h>
#include <rpm/rpmprob.h>
#include <rpm/rpmps.h>
#include <rpm/rpmsq.h>
#include <rpm/rpmstrpool.h>
#include <rpm/rpmsw.h>
#include <rpm/rpmtag.h>
#include <rpm/rpmtd.h>
#include <rpm/rpmte.h>
#include <rpm/rpmtypes.h>
#include <rpm/rpmurl.h>
#include <rpm/rpmutil.h>
#include <rpm/rpmvf.h>

// TODO: figure out why we can't include these headers (cyclic dependency)
//#include <rpm/rpmbuild.h>
//#include <rpm/rpmcallback.h>
//#include <rpm/rpmcli.h>
//#include <rpm/rpmfc.h>
//#include <rpm/rpmlib.h>
//#include <rpm/rpmspec.h>

// TODO: figure out why bindgen_test_layout_max_align_t fails with these headers
//#include <rpm/rpmbase64.h>
//#include <rpm/rpmfi.h>
//#include <rpm/rpmfileutil.h>
//#include <rpm/rpmkeyring.h>
//#include <rpm/rpmpgp.h>
//#include <rpm/rpmsign.h>
//#include <rpm/rpmstring.h>
//#include <rpm/rpmts.h>
