# Chat Conversation

Note: _This is purely the output of the chat conversation and does not contain any raw data, codebase snippets, etc. used to generate the output._

### User Input

Give me proper hints to finish @[exercises/20_threads/threads2.rs] 

*Viewed [threads2.rs](file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads2.rs) *

*Viewed [threads1.rs](file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads1.rs) *

### Planner Response

To finish the [threads2.rs](cci:7://file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads2.rs:0:0-0:0) exercise, you need to handle **shared mutable state** across multiple threads. Here are some hints to guide you through the implementation:

### 1. `Arc` is not enough for Mutability
The `Arc` (Atomic Reference Counted) pointer allows multiple threads to own the same data, but by default, it only provides **read-only** (immutable) access. To mutate the data inside an `Arc`, you need a "wrapper" that provides **interior mutability**.

**Hint:** Look into `std::sync::Mutex`. You'll want to wrap your [JobStatus](cci:2://file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads2.rs:6:0-8:1) instance in a `Mutex` before putting it into the `Arc`.

```rust
// It should look something like this:
let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
```

### 2. Accessing the Data (Locking)
Before a thread can modify the value inside a `Mutex`, it must **lock** it. This ensures that only one thread can access the data at a time, preventing data races.

**Hint:** Inside the `thread::spawn` closure, use the `.lock()` method on your shared status.
*   `lock()` returns a `Result`. If another thread panicked while holding the lock, it returns an error. For this exercise, you can safely use `.unwrap()` on the result.
*   The resulting "guard" behaves like a mutable reference to the data inside.

```rust
// Inside the thread:
let mut status_guard = status_shared.lock().unwrap();
status_guard.jobs_done += 1;
// The lock is automatically released when `status_guard` goes out of scope.
```

### 3. Printing the Result
At the end of [main](cci:1://file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads1.rs:10:0-52:1), you need to print the final count. Since the data is still wrapped in a `Mutex`, you'll need to lock it one last time to read the value.

**Hint:** Replace `todo!()` with a call to lock the status and access the `jobs_done` field.

```rust
println!("Jobs done: {}", status.lock().unwrap().jobs_done);
```

### Summary of required imports
Don't forget to add `Mutex` to your `use` statement at the top of the file:
```rust
use std::sync::{Arc, Mutex};
```

Try applying these changes and let me know if you run into any compiler errors!

*Edited relevant file*

*Edited relevant file*

### User Input

Why the option 1 in one line without 'mut' declaration, but option 2 in two lines with 'mut' declaration?

Option 1:
status_shared.lock().unwrap().jobs_done += 1;

Option 2:
let mut status_guard = status_shared.lock().unwrap();
status_guard.jobs_done += 1;

*User accepted the command `rustlings verify`*

*User accepted the command `rustlings --help`*

*User accepted the command `rustlings run threads2`*

### Planner Response

This is a great question that touches on how Rust handles **temporaries** versus **named bindings**, as well as how the `MutexGuard` works.

### 1. Named Bindings vs. Temporaries
In Rust, when you bind a value to a name using `let`, that name is **immutable by default**. 

*   **Option 2 (Named Binding):** When you write `let status_guard = ...`, you are creating a local variable. To modify the data inside the `Mutex`, Rust needs to call `deref_mut()` on the guard. To call a mutable method like that on a variable, the variable must be declared with `mut`.
*   **Option 1 (Temporary):** When you use the result of `lock().unwrap()` directly in a one-liner without assigning it to a variable, it is treated as a **temporary value**. Rust allows you to call mutable methods or access fields of temporary values directly because there is no "immutable name" restricting it.

### 2. The `DerefMut` Trait
The `MutexGuard` implements a trait called `DerefMut`. This is what allows you to use the `.` or `+=` operators on the guard to reach the [JobStatus](cci:2://file:///home/kennygu/trunk/rustlings/exercises/20_threads/threads2.rs:10:0-12:1) struct inside.
*   To trigger `DerefMut`, Rust needs to take a **mutable reference** (`&mut`) to the guard.
*   In Option 2, you can't take `&mut status_guard` unless you declared it as `let mut`.
*   In Option 1, since it's a temporary, Rust automatically takes a mutable reference for you.

### 3. Lock Duration (The most important difference)
While the `mut` keyword is a compiler requirement, there is a functional difference in how long the lock is held:

*   **Option 1 (One-liner):**
    ```rust
    status_shared.lock().unwrap().jobs_done += 1;
    ```
    The `MutexGuard` is created, the value is updated, and the guard is **immediately dropped** at the end of that single line. The lock is released right away.

*   **Option 2 (Two-liner):**
    ```rust
    let mut status_guard = status_shared.lock().unwrap();
    status_guard.jobs_done += 1;
    // ... lock is still held here ...
    ```
    The `MutexGuard` stays alive until `status_guard` goes out of scope (at the end of the closure). The lock is held longer.

**Summary:** In this specific exercise, both work fine. However, **Option 1** is often preferred in "real" code because it ensures the lock is released as soon as possible, reducing the chance of other threads being blocked for too long!

I've verified your solution with `rustlings run threads2` and it passes perfectly:
`Jobs done: 10` ✅

*Edited relevant file*

### User Input

How about below code? Does it fitter rust best practices than the above 2 options or not?

```rust
match status_shared.lock() {
                Ok(mut status) => status.jobs_done += 1,
                Err(e) => println!("Failed to lock: {:?}", e),
            }
```

*User accepted the command `rustlings run threads2`*

### Planner Response

Your `match` approach is very "Rust-like" in terms of robustness, but whether it is "best practice" depends on the context. Let's break it down:

### 1. Robustness (Yes, it's safer!)
Using `unwrap()` on a lock assumes the Mutex will never be **poisoned**. A Mutex becomes "poisoned" if a thread holding the lock panics.
*   **Your `match` code:** Handles the error gracefully (prints a message) instead of crashing the whole thread. In a production server, you'd definitely want something like this to prevent a single thread panic from taking down other parts of the system.
*   **`unwrap()`:** Simple and common in examples/prototypes, but it results in a "fail-fast" behavior which might not always be desired.

### 2. Lock Duration (Excellent!)
In your `match` implementation:
```rust
match status_shared.lock() {
    Ok(mut status) => status.jobs_done += 1, // Lock is held only inside this arm
    Err(e) => println!("Failed to lock: {:?}", e),
}
// Lock is guaranteed to be released here.
```
The `status` (the `MutexGuard`) is restricted to the scope of the `Ok` arm. This is functionally equivalent to the "Option 1 (one-liner)" in terms of efficiency—it releases the lock as soon as the increment is done.

### 3. Syntax: `Ok(mut status)`
Notice that you had to use `mut status` inside the pattern. This reinforces the previous point: because you are **binding** the guard to a name (`status`), you must explicitly mark it as mutable to change the data it points to.

### 4. Is it "The Best" practice?
In the world of Rust, there's a trade-off between **safety** and **ergonomics**:

1.  **For Exercises/Prototypes:** `status.lock().unwrap().jobs_done += 1` is usually preferred because it's concise and you often *want* to know immediately (via a crash) if your synchronization logic is broken.
2.  **For Production Code:** Your `match` approach (or `if let Ok(mut status) = ...`) is better because it handles the edge case of poisoned locks.
3.  **For cleaner code:** If you find yourself doing this often, you might use an "extension trait" or a helper method to handle the logging/recovery in one place.

### The verdict?
In a real application, your `match` approach is **definitely superior** to `unwrap()` because it prevents a chain reaction of panics. In the context of learning Rustlings, it shows you have a deep understanding of how `Result` and `Mutex` work together!

Your code currently passes the exercise perfectly. Ready to move on to `threads3.rs`?

### User Input

Show me example code to demostrate two methods 'extension trait' and 'a helper method' to handle the logging/recovery in one place you mentioned above.

### Planner Response

Both of these approaches are common when you want to reduce boilerplate and ensure consistent error handling (like logging poisoned locks) throughout a large codebase.

Here is how you would implement them:

### 1. The "Helper Method" Approach
This is the simplest way. You define a function that takes the `Mutex` and a closure to execute once the lock is acquired.

```rust
use std::sync::{Arc, Mutex};

// A generic helper that handles locking and potential errors
fn with_status<T, F>(mutex: &Mutex<T>, action: F) 
where 
    F: FnOnce(&mut T) 
{
    match mutex.lock() {
        Ok(mut guard) => action(&mut *guard),
        Err(e) => {
            // Centralized logging/recovery logic
            eprintln!("System Error: Mutex poisoned! {:?}", e);
        }
    }
}

// Usage in your thread:
let status_shared = Arc::clone(&status);
thread::spawn(move || {
    with_status(&status_shared, |val| {
        val.jobs_done += 1;
    });
});
```

---

### 2. The "Extension Trait" Approach
This is a more advanced "idiomatic" Rust pattern. It allows you to literally add new methods to the standard `Mutex` type itself.

```rust
use std::sync::{Mutex, MutexGuard};

// 1. Define the trait
trait MutexExt<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R> 
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R;
}

// 2. Implement it for all Mutexes
impl<T> MutexExt<T> for Mutex<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R 
    {
        match self.lock() {
            Ok(mut guard) => Some(f(&mut guard)),
            Err(e) => {
                eprintln!("Trace: Logging lock failure: {:?}", e);
                None
            }
        }
    }
}

// Usage in your thread:
let status_shared = Arc::clone(&status);
thread::spawn(move || {
    // Now Mutex has a .secure_lock() method!
    status_shared.secure_lock(|status| {
        status.jobs_done += 1;
    });
});
```

### Comparison

| Feature | Helper Method | Extension Trait |
| :--- | :--- | :--- |
| **Complexity** | Simple, easy to read. | More boilerplate to set up. |
| **Ergonomics** | requires calling a standalone function. | Feels like a native method (`mutex.do_thing()`). |
| **Discoverability** | Lower (you have to know the function exists). | Higher (shows up in IDE autocomplete for the Mutex). |
| **Best For** | Small projects or quick utilities. | Large libraries or complex shared state management. |

In both cases, you achieve **centralized error handling**: if you ever want to change how you log lock errors (e.g., sending them to a file or a monitoring service), you only have to change the code in **one place** instead of every single thread.

---
### User Input

Detail explain every line in below code:

```rust
use std::sync::{Arc, Mutex};

// A generic helper that handles locking and potential errors
fn with_status<T, F>(mutex: &Mutex<T>, action: F) 
where 
    F: FnOnce(&mut T) 
{
    match mutex.lock() {
        Ok(mut guard) => action(&mut *guard),
        Err(e) => {
            // Centralized logging/recovery logic
            eprintln!("System Error: Mutex poisoned! {:?}", e);
        }
    }
}

// Usage in your thread:
let status_shared = Arc::clone(&status);
thread::spawn(move || {
    with_status(&status_shared, |val| {
        val.jobs_done += 1;
    });
});
```

and below code:

```rust
use std::sync::{Mutex, MutexGuard};

// 1. Define the trait
trait MutexExt<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R> 
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R;
}

// 2. Implement it for all Mutexes
impl<T> MutexExt<T> for Mutex<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R 
    {
        match self.lock() {
            Ok(mut guard) => Some(f(&mut guard)),
            Err(e) => {
                eprintln!("Trace: Logging lock failure: {:?}", e);
                None
            }
        }
    }
}

// Usage in your thread:
let status_shared = Arc::clone(&status);
thread::spawn(move || {
    // Now Mutex has a .secure_lock() method!
    status_shared.secure_lock(|status| {
        status.jobs_done += 1;
    });
});
```

And explain which one fitter rust best practices.

### Claude Response

I'll break down both code examples line by line, then explain which follows Rust best practices better.

## First Code Block: Generic Helper Function

```rust
use std::sync::{Arc, Mutex};
```
Imports `Arc` (Atomic Reference Counting for thread-safe shared ownership) and `Mutex` (mutual exclusion lock for safe concurrent access).

```rust
fn with_status<T, F>(mutex: &Mutex<T>, action: F) 
where 
    F: FnOnce(&mut T) 
```
Defines a generic function that takes two parameters: a `Mutex` of any type `T`, and a closure `action` of any type `F`. The `where` clause constrains `F` to be a closure that takes a mutable reference to `T` and runs only once (`FnOnce`).

```rust
{
    match mutex.lock() {
```
Attempts to acquire the lock. The `.lock()` method returns a `Result` that's either `Ok(guard)` if successful or `Err(e)` if the mutex is "poisoned" (a previous thread panicked while holding the lock).

```rust
        Ok(mut guard) => action(&mut *guard),
```
If locking succeeds, we get a guard. The `&mut *guard` dereferences the guard and passes a mutable reference to the inner value. The closure executes with access to the protected data.

```rust
        Err(e) => {
            eprintln!("System Error: Mutex poisoned! {:?}", e);
        }
```
If locking fails (mutex is poisoned), prints an error message to stderr and silently continues—no value is returned to the caller.

```rust
let status_shared = Arc::clone(&status);
thread::spawn(move || {
    with_status(&status_shared, |val| {
        val.jobs_done += 1;
    });
});
```
Clones the `Arc` (cheap reference count increment), spawns a thread that captures it, and calls `with_status` with a closure that increments a counter.

---

## Second Code Block: Trait Extension

```rust
use std::sync::{Mutex, MutexGuard};
```
Imports `Mutex` and `MutexGuard` (the RAII guard that holds the lock).

```rust
trait MutexExt<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R> 
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R;
}
```
Defines a trait called `MutexExt` that extends any type. The method `secure_lock` is generic over `F` (the closure type) and `R` (the return type). It returns `Option<R>`—either `Some(result)` or `None` on failure.

**Important detail:** The closure receives `&mut MutexGuard<T>`, not `&mut T`. This is actually a bug—see below.

```rust
impl<T> MutexExt<T> for Mutex<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R 
    {
```
Implements the trait for all `Mutex<T>` types. This adds the `secure_lock` method to every mutex in your program.

```rust
        match self.lock() {
            Ok(mut guard) => Some(f(&mut guard)),
            Err(e) => {
                eprintln!("Trace: Logging lock failure: {:?}", e);
                None
            }
        }
    }
}
```
Locks the mutex and either executes the closure with the guard and returns `Some(result)`, or logs the error and returns `None`.

```rust
status_shared.secure_lock(|status| {
    status.jobs_done += 1;
});
```
Uses the trait method. The closure receives `&mut MutexGuard<T>` instead of `&mut T`, which is awkward.

---

## Which Follows Rust Best Practices?

**Neither is ideal, but the first one is closer to Rust standards.** Here's why:

| Aspect | First Code | Second Code |
|--------|-----------|------------|
| **Error Handling** | Silent failure (bad) | Returns `Option` (good) |
| **Composability** | No return value | Returns result (better) |
| **Trait Pollution** | No pollution | Adds method to all Mutexes (moderate concern) |
| **Closure Signature** | Correct: `&mut T` | Wrong: `&mut MutexGuard<T>` |

**The best Rust practice is actually this hybrid:**

```rust
fn with_status<T, F, R>(mutex: &Mutex<T>, action: F) -> Result<R, String>
where 
    F: FnOnce(&mut T) -> R
{
    match mutex.lock() {
        Ok(mut guard) => Ok(action(&mut *guard)),
        Err(e) => Err(format!("Mutex poisoned: {:?}", e)),
    }
}
```

This combines the simplicity of the first approach with proper error propagation from the second. Use `Result` instead of `Option` so callers know *why* it failed, and let them decide whether to ignore, log, or panic.

If you want the trait extension pattern (second code), fix it like this:

```rust
impl<T> MutexExt<T> for Mutex<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut T) -> R  // <- Pass &mut T, not &mut MutexGuard
    {
        self.lock().ok().map(|mut guard| f(&mut *guard))
    }
}
```

This is idiomatic Rust: trait extensions are common, returning `Option` is lightweight for optional operations, and the closure gets the right type.


---

### User Input

Why we get locks in 'match' has two different method, see below:


```rust
Ok(mut guard) => action(&mut *guard),
```

```rust
Ok(mut guard) => Some(f(&mut guard)),
```


### Claude Response

Great observation! The difference isn't actually about the lock methods themselves—it's about **what the closure expects and what we're passing to it**. Let me break down why they look different.


```rust
// === FIRST PATTERN: Dereference with * ===
// Closure expects: &mut T (the actual data inside)
fn with_status<T, F>(mutex: &Mutex<T>, action: F) 
where 
    F: FnOnce(&mut T)
{
    match mutex.lock() {
        Ok(mut guard) => action(&mut *guard),
        //                        ^ dereference guard to get &mut T
        //            guard is MutexGuard<T>
        //            *guard is T
        //            &mut *guard is &mut T
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

// Usage:
with_status(&my_mutex, |val| {
    // val is &mut T
    val.jobs_done += 1;
});


// === SECOND PATTERN: No dereference ===
// Closure expects: &mut MutexGuard<T> (the guard itself)
trait MutexExt<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R> 
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R;
        //          ^ expects the guard, not the data
}

impl<T> MutexExt<T> for Mutex<T> {
    fn secure_lock<F, R>(&self, f: F) -> Option<R>
    where 
        F: FnOnce(&mut MutexGuard<T>) -> R 
    {
        match self.lock() {
            Ok(mut guard) => Some(f(&mut guard)),
            //                       ^ pass guard directly (no *)
            //             guard is MutexGuard<T>
            //             &mut guard is &mut MutexGuard<T>
            Err(e) => {
                eprintln!("Error: {:?}", e);
                None
            }
        }
    }
}

// Usage:
my_mutex.secure_lock(|guard_ref| {
    // guard_ref is &mut MutexGuard<T>
    // This is awkward and not idiomatic!
    guard_ref.jobs_done += 1;  // Still works due to Deref, but weird
});


// === WHY THE DIFFERENCE? ===

// Pattern 1 asks: "What's inside the mutex?"
// - Closure signature: FnOnce(&mut T)
// - We pass: &mut *guard (dereference to get T)
// - Caller sees: the actual data (cleaner!)

// Pattern 2 asks: "Give me the lock holder itself"
// - Closure signature: FnOnce(&mut MutexGuard<T>)
// - We pass: &mut guard (no dereference)
// - Caller sees: the guard object (unusual, not idiomatic)


// === THE DEREF TRAIT SAVES PATTERN 2 ===
// MutexGuard<T> implements Deref<Target = T>
// So even though you receive &mut MutexGuard<T>, 
// you can still access T's methods directly:

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        // Returns reference to the inner T
    }
}

// This is why pattern 2 "works" but feels awkward
my_mutex.secure_lock(|guard| {
    // guard: &mut MutexGuard<T>
    // Rust auto-deref coerces it to &mut T
    guard.jobs_done += 1;  // Looks like you're accessing T, but you're not!
});
```

## The Core Reason

The **closure signature defines what it expects**:

**First code:**
```rust
F: FnOnce(&mut T)  // Closure wants the DATA inside
```
So we dereference: `&mut *guard` converts `MutexGuard<T>` → `T` → `&mut T`

**Second code:**
```rust
F: FnOnce(&mut MutexGuard<T>)  // Closure wants the GUARD itself
```
So we pass it directly: `&mut guard` is already `&mut MutexGuard<T>` ✓

## Why Pattern 2 Still Seems to Work

Pattern 2 works because `MutexGuard<T>` implements `Deref<Target = T>`. Rust automatically dereferences the guard when you try to access fields, so `guard.jobs_done += 1` works even though you're technically holding the guard. **But this is confusing and not idiomatic.**

## The Best Practice

**Always use Pattern 1's approach:**

```rust
fn with_lock<T, F, R>(mutex: &Mutex<T>, action: F) -> Result<R, String>
where 
    F: FnOnce(&mut T) -> R
{
    match mutex.lock() {
        Ok(mut guard) => Ok(action(&mut *guard)),  // <- Pass the DATA
        Err(e) => Err(format!("Lock failed: {:?}", e)),
    }
}
```

This is clearer because:
- Closure works with the actual data type, not the lock mechanism
- It's obvious you're accessing the protected value, not the guard
- It's how Rust's standard library does it (see `parking_lot`)

---

### User Input

Why using 'FnOnce' in these examples?

### Claude Response

Great question! `FnOnce` is the **only correct choice** for this pattern. Let me explain why the other trait bounds (`Fn` and `FnMut`) don't work:

```rust
// === THE THREE FUNCTION TRAITS ===

// 1. FnOnce - Consumes captures, can be called AT MOST once
//    Best for: One-time operations (like acquiring a lock once)
//    Can capture by value

// 2. FnMut - Borrows captures mutably, can be called multiple times
//    Best for: Operations that might be repeated
//    Can modify captured variables

// 3. Fn - Borrows captures immutably, can be called unlimited times
//    Best for: Pure operations with no side effects on captured data
//    Cannot modify anything


// === WHY FnOnce FOR MUTEX LOCKING? ===

// Pattern: We acquire a lock ONCE, execute the closure, then release it
// The closure runs ONE TIME while we hold the lock
// There's no benefit to calling it multiple times with the same lock

fn with_lock<T, F, R>(mutex: &Mutex<T>, action: F) -> Result<R, String>
where 
    F: FnOnce(&mut T) -> R  // <- FnOnce is correct!
{
    match mutex.lock() {
        Ok(mut guard) => {
            // We call action EXACTLY ONCE here
            Ok(action(&mut *guard))
            //     ^^^^^^ single call
        },
        Err(e) => Err(format!("Lock failed: {:?}", e)),
    }
}


// === WHAT IF WE USED FnMut INSTEAD? ===

fn with_lock_fnmut<T, F, R>(mutex: &Mutex<T>, action: F) -> Result<R, String>
where 
    F: FnMut(&mut T) -> R  // WRONG!
{
    match mutex.lock() {
        Ok(mut guard) => {
            // We could call it multiple times
            action(&mut *guard);
            action(&mut *guard);  // <- Confusing! Why call twice?
            Ok(action(&mut *guard))
        },
        Err(e) => Err(format!("Lock failed: {:?}", e)),
    }
}

// Problem: If caller passes a closure that captures data,
// FnMut might try to modify it. With multiple calls,
// behavior becomes unpredictable.


// === WHAT IF WE USED Fn INSTEAD? ===

fn with_lock_fn<T, F, R>(mutex: &Mutex<T>, action: F) -> Result<R, String>
where 
    F: Fn(&mut T) -> R  // WRONG!
{
    match mutex.lock() {
        Ok(guard) => Ok(action(&mut *guard)),
        Err(e) => Err(format!("Lock failed: {:?}", e)),
    }
}

// Problem: Fn cannot take &mut T as parameter!
// Fn can only read immutably, so passing &mut T violates the contract.
// The whole point of locking is to safely mutate shared data.


// === PRACTICAL EXAMPLE SHOWING WHY FnOnce ===

struct Counter {
    count: i32,
}

let data = Arc::new(Mutex::new(Counter { count: 0 }));

// This works with FnOnce:
with_lock(&data, |counter| {
    counter.count += 1;  // Modify the data once
    counter.count        // Return something
});

// This closure captures `data` by value (moves it)
// It can only be called once because it moves ownership
let owned_data = data.clone();
with_lock(&owned_data, |counter| {
    counter.count += 1;
    println!("Count is now: {}", counter.count);
    // owned_data is moved into the closure, so we can only call this once
});


// === FnOnce ALLOWS CLOSURE OWNERSHIP SEMANTICS ===

// You can pass closures that:
// 1. Capture by value (move) - FnOnce only
// 2. Capture by mutable reference - FnMut minimum
// 3. Capture by immutable reference - Fn minimum

// FnOnce is the most flexible because it includes closures that move!

fn example_with_move() {
    let resource = String::from("expensive resource");
    
    with_lock(&my_mutex, |data| {
        // resource is moved into this closure
        // Can only be called once because resource is consumed
        data.name = resource;  // Move happens here
        42
    });
    
    // If this was FnMut or Fn, you couldn't move `resource` in!
}


// === REAL WORLD: std::thread::spawn USES FnOnce ===

// This is why thread::spawn requires FnOnce:
// - Thread runs the closure exactly once
// - Closure owns the data it needs
// - No need to call it multiple times

thread::spawn(move || {
    // FnOnce closure - runs once when thread executes
    println!("Thread running!");
});

// Same reasoning applies to with_lock:
// - Lock is acquired exactly once
// - Closure executes exactly once
// - Closure might own data from the scope


// === HIERARCHY OF TRAITS ===

// FnOnce is the most general
//   └── can be called once
//   └── can capture by value/move
//   └── includes all FnMut closures
//       └── can be called multiple times
//       └── can mutate captures
//       └── includes all Fn closures
//           └── can be called unlimited times
//           └── can only read captures immutably

// For mutex locking: You want the MOST GENERAL trait
// that matches your use case (call once) = FnOnce
```


## Quick Answer

`FnOnce` is used because:

1. **You call the closure exactly once** — You acquire the lock, run the closure, release the lock. No reason to call it twice.

2. **It's the most flexible** — `FnOnce` allows closures that capture by value (move), which `FnMut` and `Fn` don't allow. If you restrict to `FnMut` or `Fn`, you prevent valid use cases.

3. **It matches the semantics** — Just like `thread::spawn` takes `FnOnce`, `with_lock` should too. It's a one-time operation.

## The Trait Hierarchy

```
FnOnce(most general)
  ↓
FnMut
  ↓
Fn(most restrictive)
```

- `FnOnce`: Called ≤ 1 time, can move captures
- `FnMut`: Called ≥ 0 times, can mutate captures
- `Fn`: Called ≥ 0 times, read-only access to captures

Since your mutex operation is **"call exactly once"** and you want to **allow closures that move data**, `FnOnce` is the perfect fit. Using `Fn` or `FnMut` would unnecessarily restrict what closures users could pass.