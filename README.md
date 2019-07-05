# imleak

[![Circle CI](https://img.shields.io/circleci/project/github/raviqqe/imleak.svg?style=flat-square)](https://circleci.com/gh/raviqqe/imleak)
[![Crates.io](https://img.shields.io/crates/v/imleak.svg?style=flat-square)](https://crates.io/crates/imleak)
[![License](https://img.shields.io/github/license/raviqqe/imleak.svg?style=flat-square)](https://opensource.org/licenses/MIT)

Immutable data structures not GC'd.

## Background

This library is created for its goals below.

- No GC code
  - Meant to be used with lower-level GC mechanism or conservative GC.
- Fully-permissive licensed
  - MIT
  - To give full control to library users.

## Alternatives

If you are looking for just a usual library of immutable data structures for daily use cases and do not care about licenses, please use one of the following.

- [im-rs](https://github.com/bodil/im-rs)
- [rpds](https://github.com/orium/rpds)

## License

[MIT](LICENSE)
