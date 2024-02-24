[![CI Status]][workflow] [![MSRV]][repo] [![Latest Version]][crates.io] [![Rust Doc Crate]][docs.rs] [![Rust Doc Main]][docs]

[CI Status]: https://img.shields.io/github/actions/workflow/status/juntyr/wobbly/ci.yml?branch=main
[workflow]: https://github.com/juntyr/wobbly/actions/workflows/ci.yml?query=branch%3Amain

[MSRV]: https://img.shields.io/badge/MSRV-1.56.0-blue
[repo]: https://github.com/juntyr/const-type-layout

[Latest Version]: https://img.shields.io/crates/v/wobbly
[crates.io]: https://crates.io/crates/wobbly

[Rust Doc Crate]: https://img.shields.io/docsrs/wobbly
[docs.rs]: https://docs.rs/wobbly/

[Rust Doc Main]: https://img.shields.io/badge/docs-main-blue
[docs]: https://juntyr.github.io/wobbly/wobbly

`wobbly` provides the single-threaded `rc::Wobbly<T>` and thread-safe `sync::Wobbly<T>` reference-counting pointers that are similar to `Weak` but provide wobbly-shared ownership of a value of type T, allocated on the heap. Unlike `Weak` pointers, a group of `Wobbly` pointers shares one owning (strong) pointer that is released when the first `Wobbly` of the group is dropped, and `Wobbly` pointers can thus keep a value alive like `std::rc::Rc` or `std::sync::Arc` but can also break cycles by being a non-owning pointer like `std::rc::Weak` or `std::sync::Weak`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Funding

`wobbly` has been developed as part of [ESiWACE3](https://www.esiwace.eu), the third phase of the Centre of Excellence in Simulation of Weather and Climate in Europe.

Funded by the European Union. This work has received funding from the European High Performance Computing Joint Undertaking (JU) under grant agreement No 101093054.
