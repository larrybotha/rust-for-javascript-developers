# Hello, Rust!

https://app.rust-for-js.dev/posts/00-hello-rust/

## Takeaways

- `rustup` is used to install the Rust toolchain, including `cargo` for crates
- `rustup` allows switching between different versions of Rust - I imagine in
  the same way as `nvm` does
- to create a new Rust project, use `cargo new [dir]`:
  ```bash
  $ cargo new awesome-project
  $ tree
  awesome-project
  ├── Cargo.toml
  └── src
  ```
- `rustc` is Rust's compiler. To compile a minimal Rust application:

  ```shell
  $ rustc src/main.rs -o target/main

  # run the resulting binary
  $ ./target/main
  ```

- `rust-analyzer` only works in properly configured projects - don't expect it
  to work if you haven't got a `Cargo.toml` and a `src` folder!
- statements ending in `!` are _not_ functions - they are macros! e.g.
  ```rust
  println!("hey there");
  ```
- string literals in Rust always use double-quotes - `"..."`
- `println!` is similar to `console.log` in JS
