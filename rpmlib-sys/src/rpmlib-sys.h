/// rpmlib-sys.h: Wrapper for rpmlib header files to be passed to bindgen
///
/// See "Using the RPM Library" section of "Chapter 15. Programming RPM with C"
/// of the Fedora RPM Guide (Draft 0.1):
///
/// https://docs.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/ch-programming-c.html#id737996
///
/// We include the header files listed in tables 16-1 and 16-2 (see below)

/** RPM sub-system header files (from Table 16-1, omitting popt.h) */
#include <rpm/rpmdb.h> // RPM database access
#include <rpm/rpmio.h> // RPM input/output routines

/** RPM data object header files (from Table 16-2) */
// #include <rpm/rpmts.h> // Transaction sets (DISABLED: see note below)
#include <rpm/rpmte.h> // Transaction elements (packages)
#include <rpm/rpmds.h> // Dependency sets
//#include <rpm/rpmfi.h> // File information (DISABLED: see note below)
#include <rpm/header.h> // Package headers

// NOTE: `rpmts.h` and `rpmfi.h` have not been included in the binding because
// they casue the `bindgen_test_layout_max_align_t` test to fail.
//
// See: https://github.com/iqlusion-io/crates/issues/11

// TODO: rpmbuild, rpmsign, and rpmspec
//
// These are not included in the binding because they include
// `/usr/include/popt.h` which presently causes the following error in the
// generated binding:
//
//     error[E0428]: the name `poptOption` is defined multiple times
//     |
//     | pub struct poptOption {
//     | --------------------- previous definition of the type `poptOption` here
//     ...
//     | pub type poptOption = *mut poptOption;
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `poptOption` redefined here
//     |
//     = note: `poptOption` must be defined only once in the type namespace of this module
//
// See: https://github.com/iqlusion-io/crates/issues/12
