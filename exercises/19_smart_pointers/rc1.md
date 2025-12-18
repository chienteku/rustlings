
### User Input

Give me hints to fix tests rc1 in @[exercises/19_smart_pointers/rc1.rs] 

*Viewed [rc1.rs](file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/rc1.rs) *

### Planner Response

To fix the [rc1](cci:1://file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/rc1.rs:36:4-106:5) test, you need to understand how `Rc<T>` handles "cloning" versus "creating new" instances.

Here are your hints:

### 1. The Meaning of `Rc`
`Rc` stands for **Reference Counted**. The whole point of `Rc` is to allow multiple things to own the **same** instance of data. 

### 2. `Rc::clone` vs. `Rc::new`
Look closely at lines 63, 68, and 73 in your current code:
- `let saturn = Planet::Saturn(Rc::new(Sun));`
- This code is creating a **brand new Sun** for Saturn.
- Because it's a new [Sun](cci:2://file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/rc1.rs:8:0-8:11), it doesn't increase the reference count of your original `sun` variable on line 39.

**Hint:** To make Saturn, Uranus, and Neptune "revolve around" the **same** [Sun](cci:2://file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/rc1.rs:8:0-8:11) as the other planets, you shouldn't create a `new` Sun. You should **clone** the reference to the existing one.

### 3. Check the Assertions
The test expects the reference count to reach **9** (line 77) and then go back down to **1** (line 106).
- Every time you use `Rc::clone(&sun)`, the count of the original `sun` goes up by 1.
- Every time a variable holding that clone (like `mercury` or `neptune`) is `drop`ed (or goes out of scope), the count goes down by 1.

### Summary of what to fix:
1. Go through the `Saturn`, `Uranus`, and `Neptune` initializations.
2. Change `Rc::new(Sun)` to `Rc::clone(&sun)`.
3. This will ensure they all share ownership of that single [Sun](cci:2://file:///home/kenggu/trunk/rustlings/exercises/19_smart_pointers/rc1.rs:8:0-8:11) instance, allowing the reference count to increment and decrement as the test expects.
