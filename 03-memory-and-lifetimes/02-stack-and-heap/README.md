# Stack and Heap

https://app.rust-for-js.dev/posts/09-stack-and-heap/

## Takeaways

### Stack

- every function call in an application pushes a frame onto a stack
- a stack from may contain:
  - local variables in a function
  - parameters to the function
- memory allocation for stack frames needs to be fixed and known at compile-time
- the stack has an upper limit on memory avaiable to it
- values that are stored in the stack _have_ to have a fixed size
- values that are dynamically sized are stored in the heap, with a reference to
  each value in the stack. References are a fixed size

### Heap

- the heap is a more free-form kind of memory region
- values that may be dynamically sized are stored in the heap; the stack stores
  pointers to these values
- values stored in the heap do not require their sizes to be known at
  compile-time
- values stored in the heap are generally stored as contiguous memory blocks.
  The larger an item in the heap gets, the more difficult and expensive it
  becomes to find contiguous blocks to store the value
- copying heap-allocated values can be expensive
- `i32` is a 32-bit signed integer - it's size is always known - 32 bits, or 4
  bytes:

  ```rust
  let my_int: i32 = 42;
  ```

  `my_int` will be allocated to the stack

- `String` is a growable type, and is thus allocated to the heap:

  ```rust
  let mut my_string = String::from("foo");

  my_string =
  ```

- stack-allocated values can be forcefully allocated to the heap using `Box`:

  ```rust
  let my_int = Box::new(123); // now heap-allocated
  ```
