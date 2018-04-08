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
    for lib in &["rpm", "rpmio"] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // `src/rpmlib-sys.h` includes all headers we generate bindings for
    let bindings = bindgen::Builder::default()
        .header("src/rpmlib-sys.h")
        .generate()
        .unwrap();

    // Write generated bindings.rs to OUT_DIR (to be included in `src/lib.rs`)
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .unwrap();
}
