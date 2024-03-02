//! [![CI Status]][workflow] [![MSRV]][repo] [![Latest Version]][crates.io]
//! [![Rust Doc Crate]][docs.rs] [![Rust Doc Main]][docs]
//!
//! [CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/wobbly/ci.yml?branch=main
//! [workflow]: https://github.com/juntyr/wobbly/actions/workflows/ci.yml?query=branch%3Amain
//!
//! [MSRV]: https://img.shields.io/badge/MSRV-1.56.0-blue
//! [repo]: https://github.com/juntyr/wobbly
//!
//! [Latest Version]: https://img.shields.io/crates/v/wobbly
//! [crates.io]: https://crates.io/crates/wobbly
//!
//! [Rust Doc Crate]: https://img.shields.io/docsrs/wobbly
//! [docs.rs]: https://docs.rs/wobbly/
//!
//! [Rust Doc Main]: https://img.shields.io/badge/docs-main-blue
//! [docs]: https://juntyr.github.io/wobbly/wobbly
//!
//! `wobbly` provides the single-threaded [`rc::Wobbly<T>`](rc::Wobbly) and
//! thread-safe [`sync::Wobbly<T>`](sync::Wobbly) reference-counting pointers
//! that are similar to `Weak` but provide wobbly-shared ownership of a value
//! of type T, allocated on the heap. Unlike `Weak` pointers, a group of
//! `Wobbly` pointers shares one owning (strong) pointer that is released when
//! the first `Wobbly` of the group is dropped, and `Wobbly` pointers can thus
//! keep a value alive like [`std::rc::Rc`] or [`std::sync::Arc`] but can also
//! break cycles by being a non-owning pointer like [`std::rc::Weak`] or
//! [`std::sync::Weak`].
//!
//! See the [`rc`] and [`sync`] modules for more details.

#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod rc;
pub mod sync;
