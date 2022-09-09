# Statements and Variables

https://app.rust-for-js.dev/posts/01-statements-and-variables/

## Takeaways

- by convention, variable names in Rust should be snake case
- `let` is used to define variables
- variables are immutable by default - reassignment throws errors
- immutable variables can be assigned after the name of the variable is
  declared:

  ```rust
  let name;
  name = "john doe"
  ```

  This is called _late initialisation_, and `clippy` warns against unneeded late
  initialisation

- mutable variables must be explicitly declared so:
  ```rust
  let mut name = "john doe"
  name = "jane doe"
  ```
- `const` is used to declare compile-time constants. Unlike in Javascript, where
  declaring a variable, such as an object, as a `const` allows one to modify
  keys on the object, in Rust the constant is truly immutable
- `const` should be used when a value is known at compile-time
- by convention, constants should be uppercase
- attempting to compile constants without providing a type will fail - Rust does
  not infer types for constants, and thus they must be provided explicitly:
  ```rust
  const IS_BOOL: bool = true;
  ```
