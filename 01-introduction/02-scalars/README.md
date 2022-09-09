# Scalars

https://app.rust-for-js.dev/posts/02-scalars/

## Takeaways

- types in programming can be scalar or compound
- Rust has 4 scalar types:
  - integers - `let integer: u64 = 5;`
  - floats - `let decimal: f32 = 1.5;`
  - characters - `let character: char = 'S';`
  - booleans - `let is_boolean: bool = true;`
- unless you have good reason to, stick to the `64` options for integers and
  floating point numbers, i.e. `i64`, `u64`, `f64`. Most machines are 64-bit
  machines nowadays, which is a useful mnemonic for remembering which types to
  stick to
- mathematical operators do not work on floats and integers without casting the
  values:
  ```rust
  let integer: u64 = 5;
  let decimal: f64 = 5.0;
  let sum: u64 = u64::from(decimal) + integer
  ```
- `::` operates similarly to the dot accessor on objects in Javascript, and
  potentially like static methods in Python...?

### Integers

- integers are have the following signed and unsigned types:
  - u[8,16,32,64,128]
  - i[8,16,32,64,128]
- unsigned integers have no sign... i.e. they are positive
- signed integers may be negative or positive, and thus have twice the number of
  possible values

### Floats

- floating point numbers may only be either `f32` or `f64`
- floats can be positive or negative
