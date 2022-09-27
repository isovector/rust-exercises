# 4. References

## Notes

Rust comes with two builtin types of references: `&_` and `&mut _`. Representation-wise,
they're not really different from references in other languages - both are pointers, with
`&_` providing immutable access and `&mut _` providing mutable one. They're constructed
using same syntax at value level.

But, what makes references in Rust interesting is how they interact with values they
reference at _compile-time_. To explain that, we'll first look into ownership.

### Ownership

Rust is an _affine_ language - that is, values in Rust can only be used once or not at
all (compare to `LinearTypes` in GHC, which have to be used exactly once). That means
that in absence of other mechanisms, passing value into function call means losing
access to it, it's no longer available in current scope. That means that given some
ordinary value in Rust, you can be absolutely sure that you're the only consumer that
can access it at that point. This makes reasoning about mutation-oriented code much
easier, because it allows one to build interfaces that appear externally pure, while
mutating arguments under the hood. At the same time, it helps with avoiding mistakes
around use-after-free or double-free.

But language that only comes with plain owned values wouldn't be very practical - even
outside of references, there're many types that are safe to clone at will, and for some
it's even cheap enough that compiler could do it implicitly (e.g. common number types).
For these purposes, Rust comes with two traits called [`Clone`] and [`Copy`].

### Cloning

(We haven't covered traits yet, but it's enough to think of them as typeclasses or
interfaces for now, they're simply implemented by types to get support for their
methods.)

[`Clone`] provides way of cloning value given a reference to it:

```rust
trait Clone {
  fn clone(&self) -> Self;
  ...
}
```

for most types, cloning simply means copying their memory elsewhere (Rust can actually
derive such impls), but some types, e.g. reference-counted values may need additional
code to make cloning safe. This trait isn't "magic" in any way - if you want to make use
of cloning, you simply call `.clone()` method on given value (thanks to rules around `.`,
you don't need to construct reference around passed-in value explicitly).

[`Copy`] is slightly more unusual:

```rust
trait Copy: Clone {}
```

First thing you may notice is that it doesn't come with any methods - it's a "marker
trait", which is pattern in Rust used to assert assumptions about types that may be needed to make some operations safe. In this case, it declares that compiler is free
to clone value by implicity copying it's memory whenever it's usage would break affinity
requirement. This makes it possible to use such values multiple times, similarly to how
it is possible in other languages.

Secondly, it's a "subtrait" of `Clone`, because `Copy`able values should be trivially
`clone`able.

### References _and_ Ownership

Alright, so we've covered owned values, and cloneable/copyable values, but references are
where ownership gets interesting. It's because references could easily break guarantees
about owned values if not restricted in some way. For example, keeping reference around
after destruction of value would once again allow for use-after-free. Similarly,
immutable references are assumed to not only restrict mutability through the reference,
they assert that underlying value is not going to be mutated elsewhere during their
lifetime - this is important, because it let's us pass access to expensive type around
while still making sure that it stays the same during that time.

(Time doesn't only apply to parallelism - when you construct record that contains
immutable references, it may be important that contents behind them stay the same
across subsequent calls to functions that make use of it - it's between those calls where
you want to be sure that rest of the code doesn't try to modify them)

At the same time, mutable references can introduce problems with aliasing - you should
not be able to create both mutable and immutable reference to a value at the same time.
Even two mutable references make things hard to track, compared to nice properties of
affine values.

And finally, these guarantees can make compiler extremely liberal with optimizations,
because if they're enforced statically, they can be statically analyzed.

So, how do we want to restrict references then? There're two rules:

- at some point in time, you can have either some amount of immutable references, or
  single mutable reference, nothing in between
- references to variable can only exist during it's lifetime

### Quick detour: Generics

Because we're soon going to get into lifetime polymorphism, it's probably good idea to
quickly cover generics in Rust. Semantically, they're very close to Haskell - there's
universal quantification and Rank-N polymoprphism (only for lifetimes at the moment).
Syntactically, they look a lot like C++:

```rust
fn constant<A, B>(a: A, _: B) -> A {
  a
}

enum Either<A, B> { Left(A), Right(B) }

fn main() {
  // Type arguments are usually inferred, but can be explicitly applied when needed.
  // So-called "turbofish syntax" `::<>` is needed instead of just `<>` to avoid
  // ambiguities around `<` operator. You can use wildcards (`_`) to write partial
  // type annotations when possible.
  constant::<usize, f64>(1, 2.);
}
```

Though, compared to Haskell, you can't have unused type parameters - if you need one in
custom datatype, you have to use dummy [`PhantomData`].

### Lifetime annotations

So, up until now, we haven't talked about any additional syntax that would be needed to
work with these properties. One could imagine that enforcement of these rules could be
completely implicit - and actually, in previous exercises, we haven't used any syntax
related to them, but problems start arising once you need to create relations between
lifetimes of different things. For example, given signature

```rust
fn(&Foo, &Bar) -> &Baz
```

There're multiple ways in which inputs and outputs may depends on each other's lifetimes:

- `&Baz` may not be related to lifetimes of function's arguments at all (it could be
  valid forever!)
- `&Baz` may specifically depend on lifetime of one of arguments (e.g. it's reference to
  field of one of them)
- `&Baz` may depend on lifetimes of both arguments (e.g. with mutable `HashMap` being 
  one of the arguments, one could store the other reference in it and return reference
  to it's entry)
- `&Foo` and `&Bar` may require one of them to outlive the other (e.g. if one of them is
  mutable and may store reference to the other)

To be able to express these relations, Rust comes with lifetime variables and bounds. For
example, mentioned cases could be expressed like this:

```rust
// Lifetime variables are prefixed with tick `'`, and must appear first in list of
// type-level variables. Usually they're written in lowercase, compared to uppercase in
// normal type variables.

// `'static` is special lifetime that means "until the end of the program" - this could
// be reference to constant, static variable or string literal.
fn case1<'a, 'b>(&'a Foo, &'b Bar) -> &'static Baz;
// Actually, it's worth mentioning that outside of `'static`, there isn't really other
// lifetime that could appear next to `Baz outside of `'a` and `'b` - unrestricted
// lifetime variable would be equivalent to it, and existential lifetime wouldn't make
// much sense.

fn case2<'a, 'b>(&'a Foo, &'b Bar) -> &'a Baz;

fn case3<'a>(&'a Foo, &'a Bar) -> &'a Baz;

fn case4<'a>(&'a Foo, &'a Bar) -> &'static Baz;
```

Though, actually, as written, these constraints may be overly restrictive - they require
lifetimes of related values to be the same, but in practice, it may be okay for one to
outlive the other, as long as it is in specific order. For that, we could use lifetime
bounds:

```rust
fn case3<'a, 'b: 'a>(&'a Foo, &'b Bar) -> &'a Baz;
```

This code says that `Bar` should live for at least as long as `Foo` - that is, `Bar` 
may outlive `Foo`, but not the opposite way.

In case of references, lifetime annotations appear after `&`, but other types may come
with lifetime variables too, with syntax similar to function declarations:

```rust
struct Foo<'a> {
  bar: &'a Bar,
  baz: Baz,
}
```

It is important to realize that when taking reference to such type, lifetime of reference
and one supplied as a parameter to `Foo` may be different - e.g. in:

```rust
&'a Foo<'b>
```

`'a` may live shorter than `'b`, e.g. when the value is borrowed only temporarily, and later used elsewhere.

Another thing worth mentioning is that there aren't "concrete lifetimes" in same way as
one can write "concrete type" - though you could imagine that in code that creates some
references, ...:

```rust
{
  let mut x = String::from("foo");
  let y = &x;
  y.chars().for_each(|c| println!("{c}"));
  let z = &mut x;
  z.push('o');
  ...
}
```

... Rust implicitly creates scopes that are then passed into functions:

```rust
{
  let mut x = String::from("foo");
  'A: {
    let y = &'A x;
    y.chars::<'A>().for_each(|c| println!("{c}"));
  }
  'B: {
    let z = &'B mut x;
    z.push::<'B>('o');
  }
  ...
}
```

Ideally trying to reduce scopes to minimum to accept as many safe programs as possible.

[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`PhantomData`]: https://doc.rust-lang.org/std/marker/struct.PhantomData.html