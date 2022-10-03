# Lifetimes

https://app.rust-for-js.dev/posts/12-lifetimes/

## Takeaways

- lifetime refers to how long the memory of a value 'lives'
- the lifetime of a value is typically its surrounding scope:

  ```rust
  fn some_fund() {
    let x = String::from("foo");

    {
      let y = String::from("bar");
    } // <= y's lifetime ends here
  } // <= x's lifetime ends here
  ```

- lifetime, as with types, can sometimes be inferred by the compiler, and at
  other times must be provided explicitly
- _lifetime annotations_ are the means to explicitly define lifetimes

  ```rust
  struct Thing <'a> {
    thing_ref: &'a String,
  }

  let val_to_ref = String::from("foo");
  let thing_a = Thing {thing_ref: &val_to_ref};
  let thing_b = Thing {thing_ref: &val_to_ref};
  ```

  Each `Thing` instance contains a reference to another value. The lifetime of
  each instance of `Thing` is now not only the scope where it is instantiated,
  but also the lifetime of the value it references. Memory allocated to each
  `Thing` instance cannot be deallocated until it is safe to deallocate the
  value that `Thing` references

- an instance of a struct that references another value cannot outlive the
  lifetime of the value it references
- conventionally, lifetime annotations are written using lowercase single
  characters
- lifetime annotations do not change the behaviour of structs - they provide
  information to the compiler on the lifetime of the dependencies of the
  struct instances
- lifetime specifiers for multiple values in a struct can be shared if the
  values inside the struct instance are defined in the same scope:

  ```rust
  fn main() {
    struct Thing<'a> {
      foo: &'a str,
      bar: &'a str,
    }

    // both values are defined in the same scope, so can share the same
    // lifetime specifier
    let thing = Thing {
      foo: "hey",
      bar: "there",
    };
  }
  ```

- if a return value of a function is derived from a parameter to the function,
  and Rust cannot infer the lifetime of the return value, the function needs a
  lifetime annotation:

  ```rust
  fn my_func'a(key: &str, hash_map: &'a HashMap<&str, &str>) -> &'a str {
    hash_map.get(key).expect("Key not found")
  }
  ```

### Outlives

- in some cases it may be important to specify that one value passed to a
  function should outlive another. This can be done using the `where` keyword:

  ```rust
  fn my_func<'a, 'b>(value_a: &'a str, value_b: &'b str) -> &'b str where 'b: 'a', {
    // ...
  }
  ```

  `where 'b: 'a` translates to _where 'b outlives 'a_

### `'static`

- a `'static` keyword exists which indicates that a value's lifetime is the
  running duration of the application - its lifetime only ends when the
  application ends

  ```rust
  static VERSION: u8 = 1;
  ```

  `VERSION` has the `'static` lifetime. If a static lifetime annotation were
  required in the context of a struct, the `'static` syntax would be required

### Option

- `Option` is an enum with 2 variants:
  - `Some` - akin to Haskell's `Just`
  - `None`
- the type of an `Option` is provided in a similar manner to how generics are
  provided to types in `TypeScript`:

  ```rust
  struct Thing {
  optional_value: Option<String>,
  }

  let thing = Thing { optional_value: Some(String::from("foo")) };
  let no_thing = Thing { optional_value: None) };
  ```

- `Some` and `None` are actually `Option::Some` and `Option::None`, but Rust
  includes the values in the prelude by default

### Hashmap

- hashmaps are similar to `Map` in Javascript, except that all key-value pairs
  must follow the same type definition
- `HashMap` is imported via `std::collections`:

  ```rust
  use std::collections::HashMap;

  let hash_map = HashMap::from([( "my_key", String::from("foo") )]);
  ```

  `HashMap`s instantiated from the `from` associated function require a slice of
  key-value tuples

- values in a `HashMap` are obtained using the `.get` method, which returns an
  `Option`
- `my_hash_map.get(key).expect("Some message")` will kill the running process -
  `.expect` in Rust should not be used in production code, unless you have a
  very good reason to kill the process
