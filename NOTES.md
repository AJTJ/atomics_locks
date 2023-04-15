## Chapter 1


### Processes vs threads
- OS's have processes, which the OS isolates from each other as much as possible.
- A program has threads, which are not isolated.

### General Notes
- println uses `std::io::Stdout::lock()` to ensure that its output stream does not get interrupted.
- The `std::thread::spawn` function is actually just a convenient shorthand for `std::thread::Builder::new().spawn().unwrap()`
  - `std::thread::Builder` allows you to set some settings
- Variable shadowing. It's good!
  - https://marabos.nl/atomics/basics.html#naming-clones
  - 

### Creating values not owned by a single thread
- statics
  ```rs
  static X: [i32; 3] = [1,2,3]`
  ```
  - this works because static is simply owned by the entire program, and is instantiated before the program even starts.
- leaking
  ```rs
  let x: &'static [i32; 3] = Box::leak(Box::new([1,2,3]));
  ```
  - from the moment it is leaked it will live forever
  - we allocate, but never drop and deallocate. If we do this forever the program will run out of memory.
- Rc
  - but is not thread-safe
- Arc (Atomically reference counted)

### Send and Sync
- Send
  - `T` is `Send` if it can be *sent* to another thread. If ownership can be transferred to another thread. `Arc<i32>` is `Send` but `Rc<i32>` is not.
- Sync
  - `T` is `Sync` if it can be *shared* with another thread. If and only if a shared reference of that type `&T`, is `Send`. `i32` is `Sync` but `Cell<i32>` is not. (`Cell<i32>` is `Send`, however.)
- All primitive types, such as `i32`, `bool`, and `str` are `Send` and `Sync`.
- `Send` and `Sync` are **auto traits**, which means they are automatically implemented on `T` based on its fields. A `struct` with fields that are all `Send` and `Sync` also has those traits.