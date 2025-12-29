
### User Input

Give me hints to finish @[exercises/19_smart_pointers/arc1.rs] 

*Viewed [arc1.rs](file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/arc1.rs) *

### Planner Response

`Arc` (Atomically Reference Counted) is the big brother of `Rc`. While `Rc` is for multiple ownership within a single thread, `Arc` is used when you need to share ownership **across different threads**.

Here are your hints to complete the exercise:

### 1. Wrapping the Data
In Rust, you can't just send a `Vec` into multiple threads because only one thing can own it at a time. To share it, you need to wrap the vector in an `Arc`.
- **Hint:** Use `Arc::new(...)` to wrap your `numbers` vector. This belongs on line 26.

### 2. Sharing with Threads
The [for](cci:1://file:///home/kenggu/trunk/rustlings/exercises/18_iterators/iterators5.rs:15:0-23:1) loop on line 30 spawns 8 threads. Each thread needs its own "handle" to the shared data. If you try to move the original `shared_numbers` into the first thread, it will be gone, and the second thread won't be able to use it!
- **Hint:** Inside the loop (line 32), you need to create a **clone** of the `Arc` for that specific thread.
- **Why?** Since `Arc` is a reference counter, "cloning" it doesn't copy the whole vector; it just creates a new pointer to the same data and increments the counter.

### 3. Syntax reminders
- To create the initial `Arc`: `Arc::new(variable)`
- To clone the `Arc`: `Arc::clone(&variable)`

### Logic flow:
1. `shared_numbers` becomes the "master" pointer.
2. `child_numbers` becomes the thread-specific pointer.
3. The `move` keyword on line 34 captures `child_numbers` and takes it into the thread, while `shared_numbers` remains available for the next iteration of the loop.

This should allow all 8 threads to look at the same 100 numbers safely at the same time!
