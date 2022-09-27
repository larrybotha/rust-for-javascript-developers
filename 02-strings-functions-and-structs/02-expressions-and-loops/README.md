# Expressions and Loops

https://app.rust-for-js.dev/posts/06-expressions-and-loops/

## Takeaways

### Conditions

- `if` conditions in Rust don't need to be wrapped in parenthesis:

  ```rust
  let x = 5;

  if x > 5 {
    println!("x is greater than 5");
  } else {
    println!("x is not greater than 5");
  }
  ```

- Rust does not cast expressions to booleans as Javascript and Python do - Rust
  always expects explicit booleans in conditions

  ```rust
  let x = [];

  if x { // invalid - must be a bool
    println!("x is truthy");
  }
  ```

  Instead, evaluate the length directly:

  ```rust
  if x.len() > 0 {
    /// ...
  }
  ```

  or better yet, if working with a list, use the `is_empty` predicate to
  evaluate whether the list is empty or not:

  ```rust
  if !x.is_empty() {
    // ...
  }
  ```

- variables can be assigned using multiple conditions, which are much more
  readable than nested ternaries in Javascript:

  ```rust
  let x = 100;
  let y = if x >= 0 && x < 10 {
    "digit"
  } else if x >= 10 && x < 20 {
    "teens"
  } else {
    "even bigger"
  };
  ```

  The conditions above use Rust's implicit return

### Loops

- Rust has a `loop` statement, which is akin to `while (true)` in Javascript:
  ```rust
  loop {
    println!("I will never stop looping!");
  }
  ```
- as in many languages, `break` will break out of a loop:

  ```rust
  let mut count = 0;

  loop {
    count += 1;

    if count >= 5 {
      break;
    }
  }
  ```

- loops can also be used for variable assignment:

  ```rust
  let count = 0;
  let x = loop {
    count += 1;

    if count >= 10 {
      break count;
    }
  };

  // count == 10
  ```

- `while` works similarly to other languages, with an expression following the
  statement:

  ```rust
  let mut count = 0;

  while count <= 10 {
    println!("count: {count}");
    count += 1;
  }
  ```

- Rust has a `for in` loop called an _iterator loop_, which operates on
  collections:

  ```rust
  let xs: [i8; 4] = [0,1,2,3];

  for x in xs {
    println!("x is {x}");
  }
  ```

  Iterables implement the `IntoIterator` trait

### Pattern matching

- Rust's pattern matching is akin to Javascript's `switch` statement, except
  that it behaves like Haskell's pattern matching when not all cases have been
  accounted for:

  ```rust
  enum Something {
    Foo,
    Bar,
    Baz
  }

  let x = Something::Foo;

  match x {
    Something::Foo => {
      println!("foo");
    }

    Something::Bar => {
      println!("bar");
    }

    _ => {
      println!("catch all");
    }
  }
  ```

- Rust has an `if let` syntax which may be useful when matching only against a
  single pattern:

  ```rust
  enum Something {
    A,
    B,
    C
  }

  let value = Something::A;

  if let Something::A = value {
    println!("value is A");
  }

  // equivalent to
  match value {
    Something::A => {
      println!("value is A");
    }

    _ => {}
  }
  ```
