//! bindgen configuration for rpmlib-sys
//!
//! For more on using rpmlib, see "Chapter 15. Programming RPM with C" from the
//! Fedora RPM Guide (Draft 0.1):
//!
//! https://docs.fedoraproject.org/en-US/Fedora_Draft_Documentation/0.1/html/RPM_Guide/ch-programming-c.html

extern crate bindgen;

use bindgen::Builder;
use std::env;
use std::path::PathBuf;

fn main() {
    // Bind to librpm.so + librpmio.so
    bind_rpmlib();

    // Bind to librpmbuild.so (if "rpmbuild" feature is enabled)
    if feature_enabled("rpmbuild") {
        bind_rpmbuild();
    }

    // Bind to librpmsign.so (if "rpmsign" feature is enabled)
    if feature_enabled("rpmsign") {
        bind_rpmsign();
    }
}

/// Bind to librpm.so + librpmio.so
fn bind_rpmlib() {
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

    generate_binding(
        "rpmlib_binding.rs",
        // TODO: whitelist types and functions we actually use
        Builder::default()
            .header("include/rpmlib.hpp")
            .blacklist_type("timex"),
    )
}

/// Bind to librpmbuild.so
fn bind_rpmbuild() {
    // Link with librpmbuild.so
    println!("cargo:rustc-link-lib=rpmbuild");

    generate_binding(
        "rpmbuild_binding.rs",
        // TODO: whitelist types and functions we actually use
        Builder::default()
            .header("include/rpmbuild.hpp")
            .blacklist_type("timex"),
    )
}

/// Bind to librpmsign.so
fn bind_rpmsign() {
    // Link with librpmsign.so
    println!("cargo:rustc-link-lib=rpmsign");

    generate_binding(
        "rpmsign_binding.rs",
        // TODO: whitelist types and functions we actually use
        Builder::default().header("include/rpmsign.hpp"),
    )
}

/// Use environment variables to determine what cargo features are enabled
fn feature_enabled(feature_name: &str) -> bool {
    let env_var = format!("CARGO_FEATURE_{}", feature_name.to_uppercase());

    if let Ok(ref enabled) = env::var(env_var) {
        if enabled == "1" {
            return true;
        }
    }

    false
}

// Write generated bindings to OUT_DIR (to be included in the crate)
fn generate_binding(filename: &str, binding: Builder) {
    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(filename);
    binding
        .generate()
        .unwrap()
        .write_to_file(output_path)
        .unwrap();
}
