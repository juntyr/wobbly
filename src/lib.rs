//! [![CI Status]][workflow] [![MSRV]][repo] [![Latest Version]][crates.io]
//! [![Rust Doc Crate]][docs.rs] [![Rust Doc Main]][docs]
//!
//! [CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/wobbly/ci.yml?branch=main
//! [workflow]: https://github.com/juntyr/wobbly/actions/workflows/ci.yml?query=branch%3Amain
//!
//! [MSRV]: https://img.shields.io/badge/MSRV-1.56.0--nightly-blue
//! [repo]: https://github.com/juntyr/const-type-layout
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
//! `wobbly`

#![deny(clippy::complexity)]
#![deny(clippy::correctness)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::style)]
#![deny(clippy::suspicious)]
// #![deny(missing_docs)] FIXME
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod rc;
pub mod sync;
