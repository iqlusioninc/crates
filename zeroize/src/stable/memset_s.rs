/// Zero out memory using `memset_s()`.
///
/// Unlike `memset()`, any call to the `memset_s()` function shall be
/// evaluated strictly, i.e. callers of `memset_s()` can safely assume that
/// it has been executed and not "optimized away" by the compiler.
pub(crate) fn secure_zero_memory(bytes: &mut [u8]) {
    #[link(name = "c")]
    extern "C" {
        fn memset_s(dest: *mut u8, dest_len: usize, byte: isize, n: usize) -> isize;
    }

    unsafe {
        assert_eq!(memset_s(bytes.as_mut_ptr(), bytes.len(), 0, bytes.len()), 0);
    }
}
