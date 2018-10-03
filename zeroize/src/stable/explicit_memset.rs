/// Zero out memory using `explicit_memset()`.
///
/// The `explicit_memset()` is a non-standard function which performs the
/// same task as `memset()`, but differs in that it guarantees that compiler
/// optimizations will not remove the operation if the compiler deduces that
/// it is "unnecessary".
pub fn secure_zero_memory(bytes: &mut [u8]) {
    #[link(name = "c")]
    extern "C" {
        fn explicit_memset(dest: *mut u8, byte: isize, n: usize);
    }

    unsafe {
        explicit_memset(bytes.as_mut_ptr(), 0, bytes.len());
    }
}
