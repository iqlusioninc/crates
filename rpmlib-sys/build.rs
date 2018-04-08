//! bindgen configuration for rpmlib-sys
//!
//! For more on using rpmlib, see "Chapter 15. Programming RPM with C" from the
//! Fedora RPM Guide (Draft 0.1):
//!
//! https://docs.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/ch-programming-c.html

extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Link with librpm.so + librpmio.so
    //
    // See "Table 16-3: Required rpm libraries" from the "Compiling and Linking
    // RPM Programs" section of "Programming RPM with C" (see above).
    //
    // We don't yet link against librpmbuild.so or librpmsign.so because bindgen
    // is having trouble generating bindings for these libraries. See
    // `rpmlib-sys.h` for more information.
    println!("cargo:rustc-link-lib=rpm");
    println!("cargo:rustc-link-lib=rpmio");

    // librpmbuild.so
    println!("cargo:rustc-link-lib=rpmbuild");

    // librpmsign.so
    println!("cargo:rustc-link-lib=rpmsign");

    // Write generated bindings.rs to OUT_DIR (to be included in `src/lib.rs`)
    let output = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");

    bindgen::Builder::default()
        .header("src/rpmlib-sys.hpp") // See this file for headers we bind
        .blacklist_type("timex") // See `lib.rs` for `struct timex` hax
        .generate()
        .unwrap()
        .write_to_file(output)
        .unwrap();
}
