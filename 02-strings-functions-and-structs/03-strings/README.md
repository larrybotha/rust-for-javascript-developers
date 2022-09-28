# Strings

https://app.rust-for-js.dev/posts/06-strings/

## Takeaways

- strings in Javascript are heap allocated, which allows them to be mutated at
  runtime
- arrays in Rust are stack allocated, which means that they have a fixed length
- strings in Rust come in two flavours:
  - `&str` - a _string slice_, which is stack allocated, and thus of fixed
    length
  - `String` - a _string_, which is heap allocated, and thus mutable. `String`
    is stored as `Vec<u8>`, a `Vec` (mutable collection) that stores UTF-8
    data

### `str`

- string literals in Rust have the type `&str`:
  ```rust
  let string_slice: &str = "Hello there";
  ```
- `&str` is part of Rust core
- `&str` may also be defined more explicitly:
  ```rust
  let explicit_str: &'static str = "explicit!";
  ```
- string literals can be written on multiple lines without any additional
  syntax:

  ```rust
  let multi_line = "
    Hey there,

    This is pretty cool
  ";
  ```

  This is like Python's `"""triple quote"""` syntax, but without the syntax

- escaping inside string literals is done with `\`, as in many languages:
  ```rust
  let x = "escaped backslash: \\";
  ```
- similar to Python's `r"some string"` syntax for raw strings, Rust also allows
  for signalling a string literal is a raw string:

  ```rust
  let raw_string = r"No need to escape \";
  ```

### `String`

- `String` is the typical go-to when working with dynamically lengthed strings
- `String` is not part of Rust core, but is available on the standard library
- a new mutable string can be created using `String::new()`:

  ```rust
  let x = String::new();
  x.str_push("ok then!");
  ```

- strings can be initialised using `String::from`:

  ```rust
  let x = String::from("hey there");
  ```

- strings can be added together, but all strings following the first string need
  to be string slices:

  ```rust
  let x = String::from("foo");
  let y = String::from("bar");

  let z = x + &y;
  ```

  `&y` is not a `&str` above - it is `&String`, but Rust coerces it to the
  correct type for us.

- `some_string.chars()` returns an iterator containing each character in the
  string:

  ```rust
  let x = String::from("hello");
  let chars = x.chars();
  ```

- `some_string.split` and `some_string.split_whitespace` split a string into an
  iterator. The iterator is lazy, so until its evaluated, no values are pulled
  out of the iterator. One way to force evaluation of an iterator is to use
  `.collect()` on the iterator:

  ```rust
  let x = String::from("hello world");
  let iterator = x.split_whitespace();
  let parts: Vec<&str> = iterator.collect();
  ```

  The type of the result of `.collect()` must be provided. A `Vec` is returned
  by `.split_whitespace` because the length of the string is not known at
  compile-time
