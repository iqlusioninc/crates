/// Zero out memory using `explicit_bzero()`.
///
/// The `explicit_bzero()` is a non-standard function which performs the
/// same task as `bzero()`, but differs in that it guarantees that compiler
/// optimizations will not remove the erase operation if the compiler
/// deduces that the operation is "unnecessary".
pub(crate) fn secure_zero_memory(bytes: &mut [u8]) {
    #[cfg_attr(not(target_os = "windows"), link(name = "c"))]
    extern "C" {
        fn explicit_bzero(dest: *mut u8, n: usize);
    }

    unsafe {
        explicit_bzero(bytes.as_mut_ptr(), bytes.len());
    }
}
