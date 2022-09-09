# Arrays and Tuples

https://app.rust-for-js.dev/posts/03-arrays-and-tuples/

## Takeaways

- Rust has 2 primitive compound types - arrays, and tuples
- both arrays and tuples have a fixed length in Rust
- Rust has no `null` or `undefined`. Because of this, arrays cannot be sparse
- arrays may only contain values of the same type
- tuples may have different types, but those types must be preserved if the
  tuple is going to be mutated
- as in Python, arrays, and tuples can be destructured
- formatting macros use the `Display` trait by default to print values. The
  `Debug` trait can be using `{:?}` for the placeholders in the macro, or
  `{arg:?}` for implicit named arguments:

  ```rust
  let xs = [1,2,3];

  println!("{:?}", xs); // or
  println!("{xs:?}");
  ```

- as in Python and Javascript, both arrays and tuples can be destructured
- `_`s in destructured values are throwaway values, as in Go
- Rust also has a rest parameter for throwing away the remaining elements in
  a list. It is represented by `..`, as opposed to Javascript's `...`.

  Unlike in Javascript, Rust's rest parameter doesn't appear to be named (not
  yet researched, but some preliminary tinkering indicates this is so). If this
  is the case, then Rust's rest parameter operates in a similar manner to
  Python's, and is useful specifically for ignoring ranges of values in arrays:

  ```rust
  let xs = [1,2,3,4,5];
  let [first, ..., last] =
  ```

### Arrays

- the types for arrays are defined using parenthesis with 2 values, delimited by
  a semicolon:
  ```rust
  let xs: [u64; 3] = [1,2,3];
  //       [1] [2]
  // 1 - type
  // 2 - length
  ```
- mutable arrays can be defined with a default value and length:
  ```rust
  // initialise a string array of length 3
  let mut xs = ["foo", 3];
  xs[0] = "bar";
  xs[1] = "baz";
  xs[2] = "quux";
  ```
- `.len` is a method on arrays, unlike in Javascript where `.length` is a
  property:
  ```rust
  let xs = [1,2,3];
  println!("length of xs: {}", xs.len());
  ```
- arrays can't be printed with formatting macros as scalars can:

  ```rust
  let xs = [1,2,3];
  println!("{}", xs); // invalid!
  ```

  This is because arrays do not implement the `Display` trait, which I imagine
  is similar to the `Show` trait in Haskell

  Instead, there is a `Debug` trait which arrays do implement, which we can
  signal to formatting macros to use using `{:?}`:

  ```rust
  let xs = [1,2,3];
  println!("{:?}", xs);
  ```

- use arrays when you know the length required
- `Vec` is a type that can grow or shrink in size, as arrays can in Javascript
  and Python

### Tuples

- as in Python, tuples may contain different types
- accessors on tuples use a dot syntax:

  ```rust
  let x_tuple = ("foo", 1, true);

  println!("{}", x_tuple.0); // => foo
  ```

- the unit tuple is represented by `()`, as in Haskell. If a function has no
  return value, it returns unit, similarly to Haskell, too.
