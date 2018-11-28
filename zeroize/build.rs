//! Support for building `explicit_bzero.c` backport for Linux w\ glibc < 2.25.

#[cfg(any(feature = "linux-backport", feature = "windows"))]
extern crate cc;
#[cfg(any(feature = "linux-backport", feature = "windows"))]
extern crate semver;

fn main() {
    #[cfg(all(feature = "linux-backport", target_os = "linux"))]
    linux::build_explicit_bzero_backport();

    #[cfg(all(feature = "windows", target_os = "windows"))]
    windows::build_explicit_bzero_shim();
}

/// Support for building the `explicit_bzero.c` backport.
/// Only used when the `linux-backport` cargo feature is enabled and the
/// installed glibc version is < 2.25 or musl-libc version is < 1.1.20.
#[cfg(all(feature = "linux-backport", target_os = "linux"))]
mod linux {
    use super::cc;
    use super::semver::Version;
    use std::process::Command;

    /// First version of glibc to support `explicit_bzero()`
    const GLIBC_WITH_EXPLICIT_BZERO: &str = "2.25.0";

    /// First version of musl-libc to support `explicit_bzero()`
    const MUSL_WITH_EXPLICIT_BZERO: &str = "1.1.20";

    enum StdLibrary {
        /// GNU C standard library
        GNU(Version),

        /// Musl C standard library
        Musl(Version),

        /// Unsupported standard library
        Unsupported,
    }

    impl StdLibrary {
        /// Build backports if necessary
        fn should_build_explicit_bzero(&self) -> Option<bool> {
            match self {
                StdLibrary::GNU(ver) => Some(ver < &Version::parse(GLIBC_WITH_EXPLICIT_BZERO).unwrap()),
                StdLibrary::Musl(ver) => Some(ver < &Version::parse(MUSL_WITH_EXPLICIT_BZERO).unwrap()),
                StdLibrary::Unsupported => None,
            }
        }

        /// Resolve the version of the installed C standard library
        fn resolve() -> Self {
            let output = Command::new("/usr/bin/ldd")
                .arg("--version")
                .output()
                .unwrap();

            let stdout = String::from_utf8(output.stdout)
                .unwrap()
                .to_ascii_lowercase();
            let stderr = String::from_utf8(output.stderr)
                .unwrap()
                .to_ascii_lowercase();

            // Check if this is GNU C standard library
            if stdout.find("glibc").is_some() || stderr.find("glibc").is_some() {
                return Self::get_glibc_version();
            }

            // Check if this is musl-libc
            if stdout.find("musl").is_some() || stderr.find("musl").is_some() {
                return Self::get_musl_version();
            }

            StdLibrary::Unsupported
        }

        /// Get the version of the GNU C standard library
        fn get_glibc_version() -> Self {
            let output = Command::new("/usr/bin/ldd")
                .arg("--version")
                .output()
                .unwrap();

            if !output.status.success() {
                panic!("/usr/bin/ldd --version exited with error: {:?}", output);
            }

            let stdout = String::from_utf8(output.stdout).unwrap();
            let info = stdout.split('\n').next().unwrap();
            let version =
                Version::parse(&(info.split(' ').last().unwrap().to_owned() + ".0")).unwrap();

            StdLibrary::GNU(version)
        }

        /// Get the version of the Musl C standard library
        fn get_musl_version() -> Self {
            let output = Command::new("/usr/bin/ldd")
                .arg("--version")
                .output()
                .unwrap();

            let stderr = String::from_utf8(output.stderr).unwrap();
            let info = stderr.split('\n').collect::<Vec<&str>>()[1];
            let version = Version::parse(info.split(' ').collect::<Vec<&str>>()[1]).unwrap();

            StdLibrary::Musl(version)
        }
    }

    /// Build `src/os/linux/explicit_bzero_backport.c` using the `cc` crate
    pub fn build_explicit_bzero_backport() {
        let stdlib = StdLibrary::resolve();

        match stdlib.should_build_explicit_bzero() {
            Some(should_build) => if should_build {
                cc::Build::new()
                    .file("src/os/linux/explicit_bzero_backport.c")
                    .compile("explicit_bzero");
            },
            None => panic!("unsupported standard library"),
        }
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
