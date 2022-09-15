# Functions and Enums

https://app.rust-for-js.dev/posts/05-functions-and-enums/

## Takeaways

### Functions

- Rust functions without return values implicitly return `()`, the unit tuple
- as with many languages, Rust has an explicit `return`:
  ```rust
  fn my_func() -> &str {
    return "foo";
  }
  ```
- Rust also has an implicit return, which is written by ommitting the `return`
  keyword, and the ending semicolon

  ```rust
  fn implicit_return() -> &str {
    "foo"
  }
  ```

  - the last _expression_ of a function in Rust may use an implicit return
  - `let` variable declarations are statements, but they evaluate expressions -
    the values that are assigned to the name `let` is defining

- anonymous functions are denoted by pipes and braces:
  ```rust
  let anon_sum = |x: u32, y: u32| { x + y };
  ```
- type annotations are also optional in anonymous functions - types are inferred
  from the types of the values passed into the function calls:

  ```rust
  let anon_sum = |x, y| { x + y }; // => |i32, i32|

  println!("sum: {}", anon_sum(1,2));
  ```

### Enums

- as in other languages, enums are used to define constructs that contain
  variants of known values
- enums are defined using the `enum` keyword, and variants are accessed using `::`:

  ```rust
  enum ImageMimeType {
    Gif,
    Png,
    Jpeg,
    Webp,
  }

  // type inferred
  let png_mime = ImageMimeType::Png;
  // explicitly typed
  let jpeg_mime: ImageMimeType = ImageMimeType::Jpeg;
  ```

- enums may also define more complex types:

  ```rust
  enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Hsl(u16, u8, u8),
  }

  let orange_rgb = Color::Rgb(251, 191, 36);
  ```

- enums variants can be simple names, constructor-like values that accept
  values, or structs

### Miscellaneous

- to indicate to the compiler that we want it automatically define a trait for
  something, we use the `#[derive(SomeTrait)]` syntax, similarly to how
  Haskell allows automatic derivation of traits:
  ```rust
  #[derive(Debug)] // equivalent to Show in Haskell
  enum MyEnum {
    // ...
  }
  ```
- `u8` can only support numbers from 0 to 255 (2^8 - 1)
  - for the rgb enum variant above, rgb only supports values from 0 to 255, so `u8` is
    appropriate
- `u16` can only support numbers from 0 to 65 535 (2^16 - 1)
  - for the hsl enum variant above, the first value represents the degrees in a
    circle, so the next best type to represent that is `u16`
