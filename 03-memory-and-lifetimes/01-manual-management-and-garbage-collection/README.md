# Manual Management and Garbage Collection

https://app.rust-for-js.dev/posts/08-manual-memory-management-and-garbage-collection/

## Takeaways

- garbage collectors count references to objects, removing them from memory when
  there are no references remaining on the next iteration of garbage
  collection
- this makes for convenient programming, but garbage collection brings a runtime
  overhead each time the garbage collector evaluates references
- additionally, memory leaks can be created by not properly dereferencing objects,
  e.g. by holding a reference inside a closure, GC may never know when the
  object has no references to it
- Rust doesn't have GC, but unlike C and C++, memory doesn't need to be managed
  manually
- the Rust compiler inserts allocation and deallocation at the correct
  locations. In order to do this, rules need to be followed when writing
  Rust, and this is where the borrow checker comes into play
