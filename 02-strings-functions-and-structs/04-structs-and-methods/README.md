# Structs and Methods

https://app.rust-for-js.dev/posts/07-structs-and-methods/

## Takeaways

### Structs

- structs in Rust are roughly equivalent to classes in Python and Javascript,
  but more similarly to structs in Go - they specify the types of values
  inhabited by the struct, and methods are defined separately from the struct,
  but as an extension of the struct
- structs are defined using the `struct` keyword:

  ```rust
  #[derive(Debug)]
  struct Thing {
    some_string: String,
    small_int: u8,
  }

  let thing = Thing {
    some_string: String::from("foo"),
    small_int: 10,
  };
  ```

- Rust has a similar feature to Javascript's property shorthand when
  instantiating structs:

  ```rust
  let some_string = String::from("foo");
  let small_int = 10;
  let thing = Thing { some_string, small_int };
  ```

- Rust's spread operator can be used to spread values when instantiating
  structs:

  ```rust
  let thing_a = Thing { some_string: String::from("foo"), small_int: 10 };
  let thing_a_clone = Thing { ..thing_a };
  ```

- as in Javascript, accessing values in structs is done with dot notation:

  ```rust
  let thing = Thing { some_string: String::from("foo"), small_int: 10 };

  println!("thing.some_string: {:?}", thing.some_string);
  ```

### Tuple Structs

- tuple structs are similar to tuples, except they have names

  ```rust
  #[derive(Debug)]
  struct TupleStruct(u8, String);

  let tuple = TupleStruct(255, String::from("howdy!"));
  ```

- Tuple structs are useful as type aliases, where it is more succinct to
  describe a specific type of value without having to access it via a property
  on an instance of a struct
- as in Python, field accessors on tuples are zero-indexed integers:

  ```rust
  struct TupleStruct(u8, String);

  let tuple = TupleStruct(255, String::from("howdy!"));
  let integer = tuple.0;
  let my_string = tuple.1;
  ```

### Methods

- for structs, Rust has methods, and associated functions
  - methods are the same as in many other languages - they are interacted with via
    an instance of a struct
  - associated functions behave like static methods in other languages - they
    are accessed on the struct itself using the `::` syntax. e.g.
    `String::from` is an associated method
- methods in Rust are defined using `impl [StructName]`:

  ```rust
  struct Foo {
    bar: String
  }

  impl Foo {
    fn say(&self, value: String) {
      let sentence = format!("{} {}", self.bar, value);

      println!("{sentence}");
    }
  }

  let my_thing = Foo { bar: String::from("foo") };

  my_thing.say("bar");
  // foo bar
  ```

  - similar to Python's classes, methods in Rust require `&self` as the first
    argument - a _reference_ to the instance, as denoted by the `&`

- associated methods are called on the struct itself, using the `::` syntax:

  ```rust
  struct Foo {
    foo: String
  }

  impl Foo {
    fn from(value: String) -> Foo {
      let result = Foo { foo: value };

      result;
    }
  }

  let thing = Foo::from(String::from("bar"));
  ```

### Other notes

- the `format!` macro returns a string, given placeholders and a variable number
  of arguments
