# canonical-path.rs

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]

[crate-image]: https://img.shields.io/crates/v/canonical-path.svg
[crate-link]: https://crates.io/crates/canonical-path
[docs-image]: https://docs.rs/canonical-path/badge.svg
[docs-link]: https://docs.rs/canonical-path/
[build-image]: https://circleci.com/gh/iqlusioninc/crates.svg?style=shield
[build-link]: https://circleci.com/gh/iqlusioninc/crates
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/iqlusioninc/crates/blob/master/LICENSE

`std::fs::Path` and `PathBuf`-like types for representing canonical
filesystem paths.

In the same way a `str` "guarantees" a `&[u8]` contains only valid UTF-8 data,
`CanonicalPath` and `CanonicalPathBuf` guarantee that the paths they represent
are canonical, or at least, were canonical at the time they were created.

[Documentation][docs-link]

## License

The **canonical-path** crate is distributed under the terms of the
Apache License (Version 2.0).

See [LICENSE] file in the `iqlusioninc/crates` toplevel directory for more
information.

[LICENSE]: https://github.com/iqlusioninc/crates/blob/master/LICENSE
