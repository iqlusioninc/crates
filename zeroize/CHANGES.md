## [0.5.0] (2018-12-24)

This release is a rewrite which replaces FFI bindings to OS-specific APIs with
a pure Rust solution.

- Use `core::sync::atomic` fences ([#146])
- Test wasm target ([#143])
- Rewrite using `core::ptr::write_volatile` ([#142])

## [0.4.2] (2018-10-12)

- Fix ldd scraper for older glibc versions ([#134])

## 0.4.1 (2018-10-12)

- Support musl-libc ([#131])
  
## 0.4.0 (2018-10-12)

- Impl `Zeroize` trait on concrete types ([#108])

## 0.3.0 (2018-10-11)

- Replace `secure_zero_memory` with `Zeroize` ([#104])

## 0.2.0 (2018-10-11)

- Add `Zeroize` trait ([#101])

## 0.1.2 (2018-10-03)

- README.md: Fix intrinsic links ([#86])

## 0.1.1 (2018-10-03)

- Documentation improvements ([#83])

## 0.1.0 (2018-10-03)

- Initial release

[0.5.0]: https://github.com/iqlusioninc/crates/pull/149
[#146]: https://github.com/iqlusioninc/crates/pull/146
[#143]: https://github.com/iqlusioninc/crates/pull/143
[#142]: https://github.com/iqlusioninc/crates/pull/142
[0.4.2]: https://github.com/iqlusioninc/crates/pull/136
[#134]: https://github.com/iqlusioninc/crates/pull/134
[#131]: https://github.com/iqlusioninc/crates/pull/131
[#108]: https://github.com/iqlusioninc/crates/pull/108
[#104]: https://github.com/iqlusioninc/crates/pull/104
[#101]: https://github.com/iqlusioninc/crates/pull/101
[#86]: https://github.com/iqlusioninc/crates/pull/86
[#83]: https://github.com/iqlusioninc/crates/pull/83
