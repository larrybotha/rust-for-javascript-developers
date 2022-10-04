# Traits

https://app.rust-for-js.dev/posts/15-traits/

## Takeaways

- structs can implement traits
- functions can accept values with specific trait constraints
- traits in Rust are similar to interfaces in Typescript - multiple types of
  values may implement a trait, and a trait specifies the minimum that needs
  to be defined in order for the trait to be implemented - just as in Haskell
- traits are defined using the `trait` keyword:

  ```rust
  trait SomeTrait {
    fn required_method_to_implement(&self) -> String;
    fn optional_method_to_implement(&self) -> String {
      "foo".to_string()
    };
  }
  ```

  - functions that need to be implemented are defined in the trait without a
    body
  - functions that may be optionally implemented are defined with a body, and
    apply to the implementor by default

- traits are implemented using the `impl` syntax:

  ```rust
  trait SomeTrait {
    fn some_method(&self);
  }

  struct Thing {
    x: String
  }

  impl SomeTrait for Thing {
    fn some_method(&self) {
      println!("x is {}", &self.x);
    }
  }
  ```

- functions can define type constraints that are Traits

  ```rust
  fn some_func<T: SomeTrait>(value: T) {
    // do something with value
  }
  ```

  Generics in Rust are defined in a similar manner on functions as tney are in
  Typescript
