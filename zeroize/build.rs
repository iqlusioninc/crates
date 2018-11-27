//! Support for building `explicit_bzero.c` backport for Linux w\ glibc < 2.25.

#[cfg(any(feature = "linux-backport", feature = "windows"))]
extern crate cc;

fn main() {
    #[cfg(all(feature = "linux-backport", target_os = "linux"))]
    linux::build_explicit_bzero_backport();

    #[cfg(all(feature = "windows", target_os = "windows"))]
    windows::build_explicit_bzero_shim();
}

/// Support for building the `explicit_bzero.c` backport.
/// Only used when the `linux-backport` cargo feature is enabled and the
/// installed glibc version is < 2.25.
#[cfg(all(feature = "linux-backport", target_os = "linux"))]
mod linux {
    use super::cc;
    use std::process::Command;

    /// First version of glibc to support `explicit_bzero()`
    const GLIBC_WITH_EXPLICIT_BZERO: &str = "2.25";

    /// Build `src/os/linux/explicit_bzero_backport.c` using the `cc` crate
    pub fn build_explicit_bzero_backport() {
        let glibc_version = get_glibc_version();

        // Hax: this probably isn't the best use of floats
        if glibc_version.is_none()
            || glibc_version.unwrap().parse::<f32>().unwrap()
                < GLIBC_WITH_EXPLICIT_BZERO.parse::<f32>().unwrap()
        {
            cc::Build::new()
                .file("src/os/linux/explicit_bzero_backport.c")
                .compile("explicit_bzero");
        }
    }

    /// Get the current glibc version (vicariously by querying `/usr/bin/ldd`)
    fn get_glibc_version() -> Option<String> {
        let output = Command::new("/usr/bin/ldd")
            .arg("--version")
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8(output.stderr).unwrap();
            let libc_info = stderr.split('\n').collect::<Vec<&str>>()[0];
            let libc_name = libc_info.split(' ').collect::<Vec<&str>>()[0];

            if libc_name != "musl" {
                panic!("/usr/bin/ldd --version exited with error: {:?}", output);
            }

            return None;
        }

        let stdout = String::from_utf8(output.stdout).unwrap();
        let info = stdout.split('\n').next().unwrap();

        Some(info.split(' ').last().unwrap().to_owned())
    }
}

/// Support for `SecureZeroMemory` (a macro found in `winnt.h`) is implemented
/// as a shim for the `explicit_bzero()` API.
#[cfg(all(feature = "windows", target_os = "windows"))]
mod windows {
    use super::cc;

    /// Build `src/os/windows/explicit_bzero_shim.c` using the `cc` crate
    pub fn build_explicit_bzero_shim() {
        cc::Build::new()
            .file("src/os/windows/explicit_bzero_shim.c")
            .compile("explicit_bzero");
    }
}
