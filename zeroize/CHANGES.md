## [0.9.2] (2019-06-28)

- README.md: add Gitter badges; update image links ([#221])

## [0.9.1] (2019-06-04)

- Impl `Zeroize` for `Option<Z: Zeroize>` ([#219])

## [0.9.0] (2019-06-04)

**NOTICE**: This release changes the default behavior of `derive(Zeroize)`
to no longer derive a `Drop` impl. If you wish to derive `Drop`, you must
now explicitly add a `#[zeroize(drop)]` attribute on the type for which you
are deriving `Zeroize`.

- Remove CPU fences ([#216])
- Remove scary language about undefined behavior ([#214])
- Bound blanket array impls on `Zeroize` instead of `DefaultIsZeroes` ([#213])
- Require `zeroize(drop)` or `zeroize(no_drop)` attributes when deriving
  `Zeroize` ([#212]).
- Support stablized 'alloc' crate ([#192])

## [0.8.0] (2019-05-20)

- Impl `Drop` by default when deriving `Zeroize` ([#188])

## [0.7.0] (2019-05-19)

- Use synstructure for custom derive ([#185])
- Add explicit array impls for `DefaultIsZeroes` ([#184])
- Remove `nightly` feature ([#183])
- Add `Zeroizing<Z>` to zeroize values on drop ([#182])

## [0.6.0] (2019-03-23)

- Add ZeroizeOnDrop marker trait + custom derive ([#168])
- Custom derive support for `Zeroize` ([#167])
- Rename `ZeroizeWithDefault` to `DefaultIsZeroes` ([#166])

## [0.5.2] (2018-12-25)

- Add `debug_assert!` to ensure string interiors are zeroized ([#156])

## [0.5.1] (2018-12-24)

- Avoid re-exporting the whole prelude ([#150])

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

[0.9.2]: https://github.com/iqlusioninc/crates/pull/224
[#221]: https://github.com/iqlusioninc/crates/pull/221
[0.9.1]: https://github.com/iqlusioninc/crates/pull/220
[#219]: https://github.com/iqlusioninc/crates/pull/219
[0.9.0]: https://github.com/iqlusioninc/crates/pull/215
[#216]: https://github.com/iqlusioninc/crates/pull/216
[#214]: https://github.com/iqlusioninc/crates/pull/214
[#213]: https://github.com/iqlusioninc/crates/pull/213
[#212]: https://github.com/iqlusioninc/crates/pull/212
[#192]: https://github.com/iqlusioninc/crates/pull/192
[0.8.0]: https://github.com/iqlusioninc/crates/pull/189
[#188]: https://github.com/iqlusioninc/crates/pull/188
[0.7.0]: https://github.com/iqlusioninc/crates/pull/186
[#185]: https://github.com/iqlusioninc/crates/pull/185
[#184]: https://github.com/iqlusioninc/crates/pull/184
[#183]: https://github.com/iqlusioninc/crates/pull/183
[#182]: https://github.com/iqlusioninc/crates/pull/182
[0.6.0]: https://github.com/iqlusioninc/crates/pull/170
[#168]: https://github.com/iqlusioninc/crates/pull/168
[#167]: https://github.com/iqlusioninc/crates/pull/167
[#166]: https://github.com/iqlusioninc/crates/pull/166
[0.5.2]: https://github.com/iqlusioninc/crates/pull/157
[#156]: https://github.com/iqlusioninc/crates/pull/156
[0.5.1]: https://github.com/iqlusioninc/crates/pull/151
[#150]: https://github.com/iqlusioninc/crates/pull/150
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
