# Adding Custom Exercises to Rustlings

This comprehensive guide will teach you how to add custom exercises and solutions to your rustlings repository, ensuring full integration with the rustlings CLI tools.

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Understanding the Rustlings Structure](#understanding-the-rustlings-structure)
4. [Step-by-Step Tutorial](#step-by-step-tutorial)
5. [Configuration Files](#configuration-files)
6. [Validation and Testing](#validation-and-testing)
7. [Troubleshooting](#troubleshooting)
8. [Best Practices](#best-practices)

---

## Overview

Rustlings uses two key configuration files to manage exercises:

1. **`info.toml`** - Defines exercise metadata for the rustlings CLI (name, directory, hints, test settings)
2. **`Cargo.toml`** - Defines binary targets for compilation

**Both files must be updated** for exercises to work with rustlings commands like `run`, `hint`, `reset`, and the interactive mode.

---

## Prerequisites

Before adding custom exercises, ensure you have:

- ✅ Rustlings installed and working (`rustlings --version`)
- ✅ Basic understanding of Rust syntax
- ✅ Familiarity with the rustlings workflow
- ✅ A rustlings repository cloned locally

---

## Understanding the Rustlings Structure

### Directory Layout

```
rustlings/
├── info.toml                    # Exercise metadata (REQUIRED)
├── Cargo.toml                   # Binary targets (auto-updated)
├── exercises/
│   ├── 00_intro/
│   │   ├── README.md           # Topic introduction
│   │   ├── intro1.rs           # Exercise file
│   │   └── intro2.rs
│   ├── 01_variables/
│   │   ├── README.md
│   │   ├── variables1.rs
│   │   └── ...
│   └── XX_your_topic/          # Your custom topic
│       ├── README.md
│       └── exercise1.rs
└── solutions/
    ├── 00_intro/
    │   ├── intro1.rs           # Solution file
    │   └── intro2.rs
    └── XX_your_topic/
        └── exercise1.rs
```

### Naming Conventions

- **Directories**: Use two-digit prefix + underscore + topic name (e.g., `24_pattern_matching`)
- **Exercise files**: Descriptive name + `.rs` extension (e.g., `patterns1.rs`)
- **Solution files**: Exact same name as exercise file
- **Binary names**: Exercise name without `.rs` (e.g., `patterns1`)
- **Solution binaries**: Exercise name + `_sol` suffix (e.g., `patterns1_sol`)

---

## Step-by-Step Tutorial

### Step 1: Choose a Topic Number and Name

Pick the next available number after existing topics:

```bash
# List existing topics
ls exercises/

# Example output:
# 00_intro  01_variables  02_functions  ...  23_conversions

# Your new topic would be:
# 24_your_topic_name
```

**Naming Tips:**
- Use lowercase with underscores
- Keep it concise but descriptive
- Follow the existing pattern

### Step 2: Create Directory Structure

```bash
# Create exercise directory
mkdir -p exercises/24_pattern_matching

# Create solution directory
mkdir -p solutions/24_pattern_matching
```

### Step 3: Create the Exercise File

Create `exercises/24_pattern_matching/patterns1.rs`:

```rust
fn main() {
    let number = 7;
    
    // TODO: Use a match expression to print whether the number is
    // "small" (0-5), "medium" (6-10), or "large" (>10)
}
```

**Critical Requirements:**
- ✅ **MUST include at least one `// TODO:` comment** - rustlings requires this
- ✅ Include a `main()` function OR test module with `#[test]` annotations
- ✅ Leave intentional gaps for learners to complete
- ✅ Focus on one concept per exercise

**Exercise File Template:**

```rust
// TODO: [Clear instruction for what the student needs to do]

fn main() {
    // Starter code here
    // Leave gaps for students to fill
}

// Optional: Add tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test code
    }
}
```

### Step 4: Create the Solution File

Create `solutions/24_pattern_matching/patterns1.rs`:

```rust
fn main() {
    let number = 7;
    
    // Using match to categorize the number
    match number {
        0..=5 => println!("small"),
        6..=10 => println!("medium"),
        _ => println!("large"),
    }
}
```

**Solution Guidelines:**
- ✅ Provide clear explanatory comments
- ✅ Show Rust best practices
- ✅ Match the exercise filename exactly
- ✅ Include the complete, working solution

### Step 5: Create Topic README (Recommended)

Create `exercises/24_pattern_matching/README.md`:

```markdown
# Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to compare values against patterns and execute code based on which pattern matches.

## Key Concepts

- `match` expressions
- Pattern syntax
- Exhaustiveness checking
- Range patterns (`0..=5`)
- Wildcard pattern (`_`)

## Further Information

- [The Rust Book - Chapter 18: Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust by Example - Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
```

### Step 6: Add Exercise to `info.toml` ⭐ **CRITICAL**

This is the **most important step** for rustlings CLI integration.

Add an entry to `info.toml`:

```toml
[[exercises]]
name = "patterns1"
dir = "24_pattern_matching"
hint = """
Use a `match` expression with range patterns like `0..=5` to categorize the number.
The `_` pattern matches anything not covered by previous patterns.

Resources:
- https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html"""
```

**Configuration Options:**

```toml
[[exercises]]
name = "exercise_name"              # Required: Exercise name without .rs
dir = "XX_topic_directory"          # Required: Directory name (no exercises/ prefix)
test = false                        # Optional: Set to false if no #[test] annotations
skip_check_unsolved = true          # Optional: Skip "already solved" check
strict_clippy = false               # Optional: Require zero clippy warnings
hint = """Multi-line hint text"""   # Required: Helpful hint for students
```

**When to use `test = false`:**
- Exercise has no `#[test]` annotations
- Exercise only has a `main()` function
- Default is `test = true` (rustlings will run tests)

**When to use `skip_check_unsolved = true`:**
- Exercise is intentionally already complete (like intro exercises)
- Working in a repository where exercises are already solved
- During development/testing

### Step 7: Update `Cargo.toml` Automatically

Run the rustlings dev command to sync `Cargo.toml`:

```bash
rustlings dev update
```

This automatically adds:
```toml
{ name = "patterns1", path = "exercises/24_pattern_matching/patterns1.rs" },
{ name = "patterns1_sol", path = "solutions/24_pattern_matching/patterns1.rs" },
```

**Manual Update (Not Recommended):**

If you prefer to update manually, add to the `bin` array in `Cargo.toml`:

```toml
bin = [
  # ... existing entries ...
  
  { name = "patterns1", path = "exercises/24_pattern_matching/patterns1.rs" },
  { name = "patterns1_sol", path = "solutions/24_pattern_matching/patterns1.rs" },
]
```

### Step 8: Validate Your Configuration

Run validation checks:

```bash
# Validate info.toml and check exercises
rustlings dev check

# If Cargo.toml needs updating:
rustlings dev update

# Verify again
rustlings dev check
```

**Expected Output:**
```
Running all exercises to check that they aren't already solved...
Progress: 97/97

Everything looks fine!
```

### Step 9: Test Your Exercise

Test all rustlings commands:

```bash
# Run the exercise
rustlings run patterns1

# Show the hint
rustlings hint patterns1

# Interactive mode
rustlings

# Check all exercises
rustlings check-all

# Reset exercise to original state
rustlings reset patterns1
```

---

## Configuration Files

### `info.toml` Structure

The `info.toml` file defines all exercise metadata:

```toml
# File header
format_version = 1

welcome_message = """Welcome message shown at start"""

final_message = """Congratulations message shown at end"""

# Exercise definitions (repeat for each exercise)
[[exercises]]
name = "exercise_name"
dir = "topic_directory"
test = false                    # Optional
skip_check_unsolved = true      # Optional
strict_clippy = false           # Optional
hint = """
Multi-line hint text.
Can include links and examples.
"""
```

### Exercise Ordering

Exercises appear in the order they're listed in `info.toml`. To insert a new exercise between existing ones, simply add it in the desired position.

### Common Configurations

**Simple exercise (no tests):**
```toml
[[exercises]]
name = "variables1"
dir = "01_variables"
test = false
hint = """Use the `let` keyword to declare a variable."""
```

**Exercise with tests:**
```toml
[[exercises]]
name = "vecs1"
dir = "05_vecs"
hint = """Create a vector using `Vec::new()` or the `vec!` macro."""
```

**Quiz exercise:**
```toml
[[exercises]]
name = "quiz1"
dir = "quizzes"
hint = """This is a quiz. No hints this time!"""
```

**Already-solved intro exercise:**
```toml
[[exercises]]
name = "intro1"
dir = "00_intro"
test = false
skip_check_unsolved = true
hint = """Enter `n` to move to the next exercise."""
```

---

## Validation and Testing

### Validation Checklist

Before committing your custom exercise:

- [ ] Exercise file has at least one `// TODO:` comment
- [ ] Solution file exists with exact same filename
- [ ] Exercise added to `info.toml`
- [ ] `test = false` set if no `#[test]` annotations
- [ ] Hint is helpful and informative
- [ ] `rustlings dev check` passes
- [ ] `rustlings dev update` executed
- [ ] `rustlings run <exercise>` works
- [ ] `rustlings hint <exercise>` shows hint
- [ ] Interactive `rustlings` shows exercise in progress

### Common Validation Errors

**Error: "Didn't find any `// TODO` comment"**
```
Solution: Add at least one // TODO: comment to your exercise file
```

**Error: "The exercise X is already solved"**
```
Solution: Add skip_check_unsolved = true to info.toml for that exercise
```

**Error: "Found the file X. Only README.md and Rust files..."**
```
Solution: Remove extra files (markdown, etc.) from exercises directory
```

**Error: "The file Cargo.toml is outdated"**
```
Solution: Run rustlings dev update
```

**Error: "has #[test] but test = false"**
```
Solution: Remove test = false from info.toml for that exercise
```

**Error: "doesn't contain any tests"**
```
Solution: Add test = false to info.toml for that exercise
```

---

## Troubleshooting

### Exercise Not Appearing in Rustlings

**Problem:** Exercise doesn't show up in `rustlings` or `rustlings run`

**Solutions:**
1. Check `info.toml` has `[[exercises]]` header before exercise definition
2. Verify exercise name matches filename (without `.rs`)
3. Run `rustlings dev update` to sync `Cargo.toml`
4. Check for typos in `name` and `dir` fields

### Hint Not Showing

**Problem:** `rustlings hint <exercise>` shows wrong hint or error

**Solutions:**
1. Verify `hint` field is properly formatted in `info.toml`
2. Check for missing closing `"""`
3. Ensure exercise name matches exactly

### Exercise Won't Compile

**Problem:** `rustlings run <exercise>` fails to compile

**Solutions:**
1. Test compilation manually: `cargo run --bin <exercise>`
2. Check for syntax errors in exercise file
3. Verify all necessary imports are included
4. Make sure the exercise is intentionally incomplete (has TODOs)

### Solution Not Found

**Problem:** "Solution for comparison: solutions/..." shows error

**Solutions:**
1. Verify solution file exists in correct directory
2. Check filename matches exercise exactly
3. Ensure `Cargo.toml` has `<exercise>_sol` binary entry

---

## Best Practices

### Exercise Design

1. **One Concept Per Exercise**
   - Focus on a single Rust concept
   - Don't combine multiple unrelated topics
   - Build complexity gradually

2. **Clear Instructions**
   - Use descriptive `// TODO:` comments
   - Explain what the student should accomplish
   - Provide context when needed

3. **Appropriate Difficulty**
   - Match difficulty to topic position
   - Early exercises should be simpler
   - Later exercises can be more challenging

4. **Good Examples**
   ```rust
   // ✅ Good: Clear instruction
   // TODO: Fix the function signature to return a String
   
   // ❌ Bad: Vague instruction
   // TODO: Fix this
   ```

### Hint Writing

1. **Be Helpful, Not Revealing**
   ```toml
   # ✅ Good hint
   hint = """
   Remember that vectors can be created with Vec::new() or the vec! macro.
   You'll need to use .push() to add elements.
   
   Resources:
   - https://doc.rust-lang.org/book/ch08-01-vectors.html
   """
   
   # ❌ Bad hint (gives away answer)
   hint = """Just write: let v = vec![1, 2, 3];"""
   ```

2. **Include Resources**
   - Link to relevant Rust Book chapters
   - Reference Rust by Example
   - Point to official documentation

3. **Progressive Hints**
   - Start with conceptual guidance
   - Provide specific pointers if needed
   - Save direct answers for solutions

### Testing

1. **Include Tests When Appropriate**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
   
       #[test]
       fn test_basic_functionality() {
           assert_eq!(my_function(5), 10);
       }
   
       #[test]
       fn test_edge_case() {
           assert_eq!(my_function(0), 0);
       }
   }
   ```

2. **Test Coverage**
   - Test normal cases
   - Test edge cases
   - Test error conditions (if applicable)

### Documentation

1. **README Files**
   - Explain the topic clearly
   - List key concepts
   - Provide learning resources
   - Keep it concise

2. **Code Comments**
   - Explain complex concepts
   - Clarify non-obvious code
   - Don't over-comment obvious things

### Version Control

1. **Commit Structure**
   ```bash
   git add exercises/24_pattern_matching/
   git add solutions/24_pattern_matching/
   git add info.toml
   git add Cargo.toml
   git commit -m "Add pattern matching exercises"
   ```

2. **What to Commit**
   - ✅ Exercise files
   - ✅ Solution files
   - ✅ README files
   - ✅ `info.toml`
   - ✅ `Cargo.toml`
   - ❌ Don't commit extra markdown files in exercises/
   - ❌ Don't commit `.rustlings-state.txt`

---

## Complete Example

Here's a complete example of adding a "Result Handling" exercise:

### 1. Create Files

**`exercises/25_result_handling/result1.rs`:**
```rust
// TODO: Complete the divide function to return a Result
// Return Ok(result) for successful division
// Return Err("Cannot divide by zero") when divisor is 0

fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    // Your code here
}

fn main() {
    // Test cases
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("Cannot divide by zero"));
    }
}
```

**`solutions/25_result_handling/result1.rs`:**
```rust
// Complete solution showing proper Result usage

fn divide(dividend: i32, divisor: i32) -> Result<i32, &'static str> {
    if divisor == 0 {
        Err("Cannot divide by zero")
    } else {
        Ok(dividend / divisor)
    }
}

fn main() {
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match divide(10, 0) {
        Ok(result) => println!("10 / 0 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert_eq!(divide(10, 0), Err("Cannot divide by zero"));
    }
}
```

**`exercises/25_result_handling/README.md`:**
```markdown
# Result Handling

The `Result<T, E>` type is used for functions that can fail. It's an enum with two variants:
- `Ok(T)` - Contains the success value
- `Err(E)` - Contains the error value

## Key Concepts

- Returning `Result` from functions
- Pattern matching on `Result`
- Error propagation with `?`
- Converting between error types

## Further Information

- [The Rust Book - Chapter 9: Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust by Example - Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
```

### 2. Add to `info.toml`

```toml
[[exercises]]
name = "result1"
dir = "25_result_handling"
hint = """
Remember that Result is an enum with Ok and Err variants.
Use an if statement to check if the divisor is zero.
Return Err("Cannot divide by zero") if it is, otherwise return Ok(result).

The function signature tells you what types to use:
- Ok variant contains an i32
- Err variant contains a &'static str
"""
```

### 3. Update and Validate

```bash
# Update Cargo.toml
rustlings dev update

# Validate
rustlings dev check

# Test
rustlings run result1
rustlings hint result1
```

---

## Summary

Adding custom exercises to rustlings involves:

1. ✅ Create exercise and solution files
2. ✅ Add `// TODO:` comments to exercises
3. ✅ Create topic README (optional)
4. ✅ Add exercise to `info.toml` with proper configuration
5. ✅ Run `rustlings dev update` to sync `Cargo.toml`
6. ✅ Validate with `rustlings dev check`
7. ✅ Test with `rustlings run`, `rustlings hint`, and interactive mode

**Key Files:**
- `info.toml` - Exercise metadata (REQUIRED)
- `Cargo.toml` - Binary targets (auto-updated)
- Exercise files - Must have `// TODO:` comments
- Solution files - Complete working solutions

**Rustlings Commands:**
- `rustlings dev check` - Validate configuration
- `rustlings dev update` - Sync Cargo.toml
- `rustlings run <exercise>` - Run specific exercise
- `rustlings hint <exercise>` - Show hint
- `rustlings` - Interactive mode

Happy teaching! 🦀
