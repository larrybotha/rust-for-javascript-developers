# Borrow Checker

https://app.rust-for-js.dev/posts/10-borrow-checker/

## Takeaways

- there are 4 rules the borrow checker enforces in order to allow code to
  compile:
  - every value in Rust must have a single owner
  - a value may have any number of immutable references to it
  - a value may have only a single mutable reference
  - if a value has a mutable reference, it may not have any immutable references
- values are dropped from memory when they go out of scope
- references that go out of scope do not result in their associated values being
  dropped from memory
- the action of creating a reference is called _borrowing_

### Ownership

- a value may only have a single owner
- ownership of values changes based on a number of rules:

  - assigning a variable to another variable will _move_ ownership if the
    memory of the referenced variable is allocated to the heap:

    ```rust
    let x_stack = "foo";
    let y_stack = x_stack; // <= ownership is not moved

    println!("{x_stack} and {y_stack} compiles - ownership has not moved");

    let x_heap = String::from("foo");
    let y_heap = x_heap; // <= ownership is moved to y_heap

    println!("
      {x_heap} and {y_heap} fails to compile -
      the ownership of x_heap's value has moved to y_heap.
      x_heap is no longer in scope
    ");
    ```

    Rust calls this _moving x_heap into y_heap_ - the pointer on the stack is not
    copied, it is considered _moved_

    This ensures that for every value that exists on the heap, there is only
    ever a single pointer to it from the stack, eliminating the possibility of
    double free errors ocurring (freeing memory to a heap-allocated value
    twice)

  - passing a variable to a function will move ownership of the underlying value
    to that function:

    ```rust
    fn my_fn(value: String) {
      println!("{value}");
    }

    let x = String::from("foo"); // <= ownership starts here

    // <= ownership is now moved to my_fn, and x is no longer in scope
    my_fn(x);
    ```

- when a value goes out of scope, it is dropped from memory
  - if the value is returned, the caller can take ownership of the returned
    value

### Immutable References

- instead of moving ownership, one can also pass around references to values:

  ```rust
  fn my_fn(value: &String) {
    println!("{value}");
  }

  let my_string = String::from("foo");
  let my_ref = &my_string;

  my_fn(my_ref); // <= no need for & - defined as a ref
  my_fn(&my_string);
  my_fn(&my_string); // <= safe to call again
  ```

- there may be any number of immutable references to a value
- another name for an immutable reference is an _immutable borrow_

### Mutable references

- mutable references allow for referencing mutable values that need to be
  mutated by reference (ew!):

  ```rust
  fn mutate_that(value: &mut String) {
    value.push_str(" we're not done here!");
  }

  let mut x = String::from("hey...");
  let y = &mut x;

  mutate_that(y);
  ```

- mutable references are only allowed on mutable values
- only a single mutable reference may exist for any particular value
- immutable references are not allowed while a value has a mutable reference

## Links and Resources

- [Rust ownership docs](http://doc.rust-lang.org/1.64.0/book/ch04-01-what-is-ownership.html#ownership-rules)
- [Rust references and borrowing docs](http://doc.rust-lang.org/1.64.0/book/ch04-02-references-and-borrowing.html)
