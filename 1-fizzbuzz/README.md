# 1. FizzBuzz

In this folder, create MR that adds binary crate `fizzbuzz`, with single `main.rs` file
that implements `fizzbuzz : fn(usize) -> String` and `main : fn()` that prints result for
first 16 values.

## Notes

- `cargo init` creates `main.rs` for you
- `return` (and `break`) are _expressions_ (with return type `!` - "never", `Void` in
  Haskell), you can use them anywhere you want
- blocks (`{ ... }`) in rust implicitly return expression on last line, if it doesn't
  have trailing `;` - otherwise it returns `()`
- syntax for function definitions is:
  ```rust
  fn function(arg1: Type1, arg2: Type2) -> ReturnType {
    ...
  }
  ```
  where `ReturnType` defaults to `()` (unit) when omitted, and body follows rules for
  blocks
- `let var = 42;` defines constant variable `var` with value `42`, `let mut` makes it
  mutable
- types of `let` bindings are inferred, but you can use `let x: T = y;` syntax if needed
- `if x { y } else { z }` (braces are required, but predicate doesn't need parens)
- `for i in 0..10 { x }` (we'll look into iterators (and range expressions) later)
- `while x { y }`
- `loop { x }` is equivalent to `while true { x }`
- pattern matching with exhaustiveness checking à la Haskell (separated by commas,
  optional after block body):
  ```rust
  match {
    A => w,
    B => { x; y }
    C => z,
  }
  ```
- all bindings can pattern match, as in Haskell (including `let` and function arguments),
  but outside of `match` they have to be irrefutable
- there's bunch of numeric types in Rust, but there's system to it:
  - signed integers start with `i` (integer), unsigned ones with `u`, floats with `f`
  - prefix is followed by size in bits (`8`, `16`, `32`, `64`, `128`), or `size` for
    architecture-specific "pointer size"
  - there're additional `NonZero_`, `Saturating<_>` and `Wrapping<_>` and `Atomic_`
    variants for respective purposes - normal versions panic in debug mode on overflow
- Rust has two types of (normal) strings - `str` and `String`
  - `str` is unsized chunk of characters, and thus is usually accessed through some
    pointer - string literals have type `&str` (or `&'static str` to be precise)
    - can be thought of as `[u8]`, but in safe API that takes care of UTF-8
  - `String` is mutable, boxed string type (comparable to `Vec<u8>`), that can be used
    to dynamically construct new strings
    - you can turn literal into `String` by wrapping it in `format!(...)` or
      `String::from(...)`, or using `.into()` method
    - to turn `String` into `&'a str` for some scope `'a`, you can simply wrap it in a
      reference:
      ```rust
      let a = String::from("test");
      let b: &str = &a;
      ```
      there's a little bit of magic involved in conversion from `String` to `str`, I'll
      explain that later
- math operators are similar as in other languages
- string formatting is done through `format!` macro - it takes formatting string and
  optional arguments separated by commas:
  ```rust
  format!("Hello {}, you're {} years old!", name, age)
  ```
  - braces represent arguments, use double braces for literal braces
  - if provided argument is plain variable, you can name it directly in braces:
    ```rust
    format!("Hello {name}, you're {age} years old!")
    ```
  - you can use numbers instead of names to refer to positional arguments out of order
  - argument can optionally come with formatting options separated by `:`:
    ```rust
    format!("{:?}", thing)
    format!("{thing:?}")
    ```
    prints `thing` using `Debug` trait (similar to Haskell's `Show`) instead of
    `Display` trait (human-readable formatting). `:#?` will print using multiline,
    indented representation (great for dumping ASTs!)
    - see https://doc.rust-lang.org/std/fmt/index.html for more options
