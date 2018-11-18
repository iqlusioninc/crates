//! stable: invoke OS-specific secure memory zeroing intrinsics

// Linux, FreeBSD, OpenBSD, DragonflyBSD: use `explicit_bzero()`
#[cfg(
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "openbsd",
        target_os = "windows", // via `explicit_bzero_shim.c`
    )
)]
mod explicit_bzero;
#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "openbsd",
    target_os = "windows"
))]
pub(crate) use self::explicit_bzero::secure_zero_memory;

// iOS, Mac OS X, Solaris: use `memset_s()`
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "solaris"
))]
mod memset_s;
#[cfg(any(
    target_os = "ios",
    target_os = "macos",
    target_os = "solaris"
))]
pub(crate) use self::memset_s::secure_zero_memory;

// NetBSD: use `explicit_memset()`
#[cfg(target_os = "netbsd")]
mod explicit_memset;
#[cfg(target_os = "netbsd")]
pub(crate) use self::explicit_memset::secure_zero_memory;

#[cfg(not(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
    target_os = "solaris",
    target_os = "windows"
)))]
compile_error!("no secure_zero_memory() implementation available for this platform");
