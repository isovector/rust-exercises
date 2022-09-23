# Rust exercises

## Cheatsheet

### Useful links

- Main page: https://rust-lang.org
- rustup (toolchain manager): https://rustup.rs/
  - `stable` is "stable" version of the compiler, `beta` are future candidates
  - I tend to use `nightly` version of compiler, because it supports unstable features
    - can be set with `rustup default nightly-DATE`
    - check https://rust-lang.github.io/rustup-components-history/ to find latest DATE
      that supports rust-analyzer
- install rust-analyzer, it helps a lot: https://rust-analyzer.github.io/
- `std` documentation: https://doc.rust-lang.org/std/index.html (`!ruststd` in Brave/DDG)
- Crate registry (very similar to Hackage): https://crates.io/ (`!crates`)
  - Alternatively, with crates sorted by categories: https://lib.rs (`!librs`)
- Crates documentation: https://docs.rs (`!docsrs`)
- When interested in concrete part of ecosystem (GUI, async, etc.), try searching for
  "rust are we _ yet" - there're often pages tracking status of these

### Cargo

- `cargo init` to create new project (see `--help` for options)
- `cargo build` to build it
  - builds in DEBUG mode by default, make sure to use `cargo build --release` to get
    realistic performance!
  - you can find binaries in `/target/{debug,release}` directory
- `cargo run` to run (main) executable
  - program flags can be added after `--`
  - specify executable name when having multiple ones
- `cargo test` to run tests

## TODO list (TODO)

- borrow checker is e v e r y w h e r e
  - but we could talk about basic principles first
    - owned values
    - references
    - mutable references
    - lifetimes
    - `'static`
- [ ] syntax refresher
- [ ] modules, visibility
- [ ] builtin types
- [ ] data declarations
- [ ] basic `std` stuff (common functions, operators, types)
- [ ] (lifetime) polymorphism
- [ ] custom traits
- [ ] overview of Rust's "typeclasspedia" (cloning, formatting, equality, ordering, ...)
- [ ] pointers and references (common smart pointers)
- [ ] error handling (`Option` and `Result`, `?` operator, `anyhow` and `thiserror`)
- [ ] iterators (`std::iter` API, custom ones)
- [ ] basic IO
- [ ] multithreading
- [ ] (de)serialization (`serde`)
- [ ] closures (`Fn_` traits)
- [ ] type synonyms, `impl` types
- [ ] `dyn` types
- [ ] macros (at least declarative ones)
- [ ] async (futures representation, `Future` trait, `.await` syntax, `async-std`)