# 2. Calculator

In this folder, create MR that introduces module `expr` with type `Expr` for simple
expressions consisting of floats (`f64`) and basic operations (addition, subtraction,
multiplication, division). Then, in the same module, create function
`eval: fn(Expr) -> f64` that evaluates these expressions and extend `main` to print
result of evaluating `(((1 + 2) * 3) / 4)`.

## Notes

- Rust has builtin module system - so no preprocessing! Modules can be created using
  `mod` declarations:
  ```rust
  mod foo {
    fn bar() {}
  }
  ```
  introduces module `foo` into scope, with member `foo::bar`. On the other hand:
  ```rust
  mod foo;
  ```
  will use file `foo.rs` in current directory (of the current file).
- Imports are done using `use` declarations:
  ```rust
  use foo::bar; // introduces `bar` into scope
  ```
  but they're not strictly needed - as long as something is `use`able, it can always
  be used fully qualified.
  You can `use` multiple members too:
  ```rust
  use foo::{bar, baz}
  ```
  At the same time, they can be nested (because modules are nested, starting from main
  module):
  ```rust
  use foo::qux::bar;
  use foo::{qux::bar, baz};
  ```
  And you can `use` both qualified module and some unqualified members using `self`:
  ```rust
  use foo::qux::{self, bar, baz}; // both `qux::bar` and `bar` in scope
  ```
  To access members of parent in nested module, you can use `super`:
  ```rust
  fn foo() {}

  mod bar {
    fn baz() {
      super::foo()
    }
  }
  ```
- instead of export lists, Rust uses visibility modifiers
  - visibility applies to modules too
  - declarations are private by default, and can only be accessed in current and nested
    modules
  - `pub` makes member completely public
  - `pub(crate)` applies to current crate
  - `pub(super)` applies to parent module
  - `pub(self)` is noop
  - `pub(in PATH)` makes member visible in ancestor PATH
- Rust has several types of data declarations:
  - `struct`s
    - with anonymous fields:
      ```rust
      struct User(Id, Name, Email);

      fn main() {
        ...
        let user = User(id, name, email);
        ...
      }
      ```
    - with named fields:
      ```rust
      struct User {
        id: Id,
        name: Name,
        email: Email,
      }

      fn main() {
        ...
        let user = User {
          id: id,
          name: name,
          email: email,
        };
      }
      ```
      Rust has equivalent of `NamedFieldPuns` too:
      ```rust
      ...
      let user = User { id, name, email };
      ...
      ```
      Fields can be accessed using dot syntax:
      ```rust
      ...
      user.id
      ...
      ```
  - `enum`s:
    ```rust
    enum Shape {
      Rectangle(f64, f64),
      Circle { radius: f64 },
    }

    fn main() {
      ...
      // `enum` constructors are qualified by type name
      let shape1 = Shape::Rectangle(1., 2.);
      use Shape::Circle;
      let shape2 = Circle { radius: 5. };
      ...
    }
    ```
    `enum` fields don't support dot syntax (so no partial selectors!)
  - Everything can be pattern-matched on!
  - There're `union`s too, but they're basically only used for C interop
    (and are `unsafe`)
- Anonymous functions are written like this:
  ```rust
  |a, b, c| a + b * c
  ```
  - as long as they don't capture any context, they're ordinary functions - we'll look
    into closures later
  - normal functions (both anonymous and named) have types resembling function
    declaration syntax:
    ```rust
    (|a, b, c| a + b * c): fn(f64, f64, f64) -> f64
    (|x| println!("{x}")): fn(f64) // return type defaults to `()`
    ```
  - all functions are first-class, you can pass them around
- Once you end up with recursive ADT, you'll need to introduce boxing to avoid type with
  infinite size - for that you can use `Box`. `Box<T>` for some `T` is pointer to owned
  value allocated on heap - we'll look into ownership later, all you need to know now is
  that you can create boxed value using `Box::new(...)` and take out it's contents using
  dereferencing: `*boxed`.