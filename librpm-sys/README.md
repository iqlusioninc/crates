# librpm-sys: bindgen wrapper to librpm

This crate provides a raw bindgen wrapper to the librpm C library, which
provides a low-level API for interacting with the details of RPM files.

This crate isn't intended to be used directly, but instead provides the
low-level unsafe binding used by the higher level librpm crate.
