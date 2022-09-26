# 3. Calculator with variables

In this folder, create MR that extends expression evaluator with support for variables.
That is, change `Expr::eval` to have signature `fn(self, &HashMap<String, f64>) -> f64`,
where second argument will provide bindings for used variables, and the function will
panic if it encounters variable with missing binding. Use this new version of `eval` to
evaluate `((x + 2) * y) / 4` with `x = 1, y = 3`.

Then, create separate private method `BinOp::to_fn(self) -> fn(f64, f64) -> f64` and
use it in place of current code that picks correct binary operation in `Expr::eval`.

## Notes

- it is possible to import all members of module using `*` wildcard - this is convenient
  for prelude-like modules (external crates sometimes come with modules called 
  `prelude`), but isn't good practice in general because of possible import ambiguities
- Rust comes with method syntax in style of OOP languages - methods can come from traits
  or type-specific `impl` blocks (we'll look into traits later)
  - type-specific methods can be defined in `impl Type` block, which can only appear in
    same module as type declaration:
    ```rust
    struct Counter(usize);

    impl Counter {
      // Methods are allowed to take `self` as a special first argument without
      // type annotation - this makes it possible to call them using method syntax:
      // ```
      // counter.get()
      // ```
      // Though, it is always possible to call them "normally" by using qualified
      // method name:
      // ```
      // Counter::get(counter)
      // ```

      // Method taking ownership of supplied value - `self: Counter`
      fn to_usize(self) -> usize {
        self.0
      }

      // "static" method - doesn't come with `self` argument, can only be called
      // qualified: `Counter::new()`.
      //
      // Defining `new` as a "smart constructor" is a common idiom.
      fn new(
      // `Self` type can be thought of as type synonym for head of `impl` block.
      ) -> Self {
        // Similarly, value `Self` is synonym for constructor of current `struct`.
        Self(0)
      }

      // Method taking value by reference - `self: &Counter`.
      fn current(&self) -> usize {
        self.0
      }

      // Method taking mutable reference to value - `self: &mut Counter`.
      fn increment(&mut self) {
        self.0 += 1
      }
    }
    ```
  - compared to C(++), Rust doesn't have `->` operator - instead, compiler will
    try to dereference given value as much as needed, trying `T`, `&T` and `&mut T` for
    every result of dereferencing along the way
    - dereferencing for non-builtins is done using [`Deref`]/[`DerefMut`] traits
      - e.g. [`String`] implements [`Deref`] with target being [`str`], which allows you
        to use [`str`]'s methods with [`String`]s
      - we'll take a closer look at traits working with references later 
  - [`HashMap`] is pretty self-descriptive - we'll look into generics later, but it can
    be used simply by specifying key and value types: `HashMap<Foo, Bar>`
    - to construct one, you can use `from` static method, which expects slice of pairs of
      values: `HashMap::from([(1, 2.), (3, 4.), ...])`
    - partial (potentially panicking) indexing can be done using indexing operator (just
      keep in mind that it expects reference to requested key): `hash_map[&1]`
    - in this exercise, `eval` will expect reference to a map, so make sure to wrap it
      in one: `(...).eval(&map)`

[`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
[`DerefMut`]: https://doc.rust-lang.org/std/ops/trait.DerefMut.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`str`]: https://doc.rust-lang.org/std/primitive.str.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html