# How to Add Custom Exercises and Solutions in Rustlings

## Overview

Rustlings uses a structured approach where exercises and solutions are defined in two key files:
1. **`info.toml`** - Defines exercise metadata (name, directory, hints, test settings) for the rustlings CLI
2. **`Cargo.toml`** - Defines binary targets for compilation

**Both files must be updated** for exercises to be fully integrated into the rustlings CLI interface.

## Directory Structure

```
rustlings/
├── info.toml               # Exercise metadata for rustlings CLI
├── Cargo.toml              # Defines all exercise and solution binaries
├── exercises/
│   ├── XX_topic_name/      # Numbered topic directories
│   │   ├── README.md       # Topic introduction and resources
│   │   ├── exercise1.rs    # Exercise files
│   │   └── exercise2.rs
│   └── quizzes/            # Quiz exercises
└── solutions/
    ├── XX_topic_name/      # Matching directory structure
    │   ├── exercise1.rs    # Solution files
    │   └── exercise2.rs
    └── quizzes/
```

## Step-by-Step Guide

### 1. **Choose a Topic Number and Name**

Topics are numbered sequentially (e.g., `00_intro`, `01_variables`, `23_conversions`). Choose the next available number or insert between existing topics.

### 2. **Create Directory Structure**

```bash
# Create exercise directory
mkdir -p exercises/24_your_topic

# Create solution directory
mkdir -p solutions/24_your_topic
```

### 3. **Create Exercise Files**

Create your exercise file in `exercises/24_your_topic/your_exercise1.rs`:

```rust
fn main() {
    // TODO: Add your exercise instructions here
    let x = 5;
    
    println!("x has the value {x}");
}
```

**Exercise File Guidelines:**
- **MUST include at least one `// TODO:` comment** - rustlings requires this to guide learners
- Include `main()` function or test modules
- Leave intentional gaps for learners to fill
- Keep exercises focused on one concept

### 4. **Create Solution Files**

Create the corresponding solution in `solutions/24_your_topic/your_exercise1.rs`:

```rust
fn main() {
    // Explanation of the solution
    let x = 5;
    
    println!("x has the value {x}");
}
```

**Solution File Guidelines:**
- Provide clear explanatory comments
- Show best practices
- Match the exercise filename exactly

### 5. **Add Topic README (Optional but Recommended)**

Create `exercises/24_your_topic/README.md`:

```markdown
# Your Topic Name

Brief explanation of the concept being taught.

## Further information

- [Relevant Rust Book Chapter](https://doc.rust-lang.org/book/...)
- Additional resources
```

### 6. **Add Exercise to info.toml** ⭐ **CRITICAL STEP**

This is **required** for rustlings CLI integration. Add an entry to `info.toml`:

```toml
[[exercises]]
name = "your_exercise1"
dir = "24_your_topic"
hint = """
Provide a helpful hint here.
Can be multi-line.
Include links to documentation if helpful."""
```

**Important Notes:**
- `name` - Exercise name without `.rs` extension
- `dir` - Directory name (without `exercises/` prefix)
- `hint` - Multi-line hint shown when user types `rustlings hint your_exercise1`
- `test = false` - Add this line if your exercise has no `#[test]` annotations (default is `true`)

### 7. **Register Exercises in Cargo.toml**

Add entries to the `bin` array in `Cargo.toml`:

```toml
bin = [
  # ... existing entries ...
  
  # Your new exercises
  { name = "your_exercise1", path = "exercises/24_your_topic/your_exercise1.rs" },
  { name = "your_exercise1_sol", path = "solutions/24_your_topic/your_exercise1.rs" },
]
```

**Important Notes:**
- Exercise name: `exercise_name`
- Solution name: `exercise_name_sol` (must have `_sol` suffix)
- Paths must be exact and relative to the repository root
- Order matters - exercises appear in the order listed

### 8. **Validate and Sync Configuration**

Run these commands to validate your setup:

```bash
# Validate info.toml and check exercises
rustlings dev check

# Update Cargo.toml to match info.toml
rustlings dev update

# Verify again after update
rustlings dev check
```

### 9. **Update Exercise Mapping (Optional)**

If your topic corresponds to a Rust Book chapter, update `exercises/README.md`:

```markdown
| your_topic             | §X.Y                |
```

### 10. **Test Your Exercises**

```bash
# Test running a specific exercise
rustlings run your_exercise1

# Test the hint system
rustlings hint your_exercise1

# Test in interactive mode
rustlings

# Check all exercises
rustlings check-all
```

## Example: Adding a Custom "Pattern Matching" Exercise

### 1. Create directories:
```bash
mkdir -p exercises/24_pattern_matching
mkdir -p solutions/24_pattern_matching
```

### 2. Create `exercises/24_pattern_matching/patterns1.rs`:
```rust
fn main() {
    let number = 7;
    
    // TODO: Use a match expression to print whether the number is
    // "small" (0-5), "medium" (6-10), or "large" (>10)
}
```

### 3. Create `solutions/24_pattern_matching/patterns1.rs`:
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

### 4. Create `exercises/24_pattern_matching/README.md`:
```markdown
# Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to compare values against patterns.

## Further information

- [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
```

### 5. Add to `info.toml`:
```toml
[[exercises]]
name = "patterns1"
dir = "24_pattern_matching"
test = false
hint = """
Use a `match` expression with range patterns like `0..=5` to categorize the number.
The `_` pattern matches anything not covered by previous patterns."""
```

### 6. Add to `Cargo.toml`:
```toml
  { name = "patterns1", path = "exercises/24_pattern_matching/patterns1.rs" },
  { name = "patterns1_sol", path = "solutions/24_pattern_matching/patterns1.rs" },
```

### 7. Validate:
```bash
rustlings dev check
rustlings dev update
```

### 8. Test:
```bash
rustlings run patterns1
rustlings hint patterns1
rustlings
```

## Key Points to Remember

1. **TODO Comments Required**: Every exercise MUST have at least one `// TODO:` comment

2. **Two Files to Update**: Both `info.toml` (for rustlings CLI) and `Cargo.toml` (for compilation) must be updated

3. **Naming Convention**: Exercise binaries don't include the directory prefix (use `patterns1`, not `24_pattern_matching_patterns1`)

4. **Solution Suffix**: Solutions must end with `_sol` in the binary name

5. **Directory Numbering**: Use two-digit prefixes (e.g., `01_`, `24_`) for proper sorting

6. **File Synchronization**: Every exercise must have a corresponding solution with the exact same filename

7. **Test Configuration**: Add `test = false` in `info.toml` if your exercise has no `#[test]` annotations

8. **Validation Commands**: Always run `rustlings dev check` and `rustlings dev update` after adding exercises

9. **Testing**: Test both the exercise and solution, and verify CLI integration with `rustlings run`, `rustlings hint`, and interactive `rustlings`

## Rustlings CLI Commands

Once integrated, your exercises can be managed with:

- `rustlings` - Interactive mode with progress tracking
- `rustlings run <exercise>` - Run a specific exercise
- `rustlings hint <exercise>` - Show hint for an exercise
- `rustlings reset <exercise>` - Reset an exercise to its original state
- `rustlings check-all` - Check all exercises
- `n` (in interactive mode) - Move to next exercise
- `h` (in interactive mode) - Show hint
- `q` (in interactive mode) - Quit

This complete integration allows rustlings to:
- Track student progress across all exercises
- Provide hints and solutions on demand
- Run exercises in a specific order
- Verify completeness with the progress bar
- Enable interactive learning workflow


## Directory Structure

```
rustlings/
├── Cargo.toml              # Defines all exercise and solution binaries
├── exercises/
│   ├── XX_topic_name/      # Numbered topic directories
│   │   ├── README.md       # Topic introduction and resources
│   │   ├── exercise1.rs    # Exercise files
│   │   └── exercise2.rs
│   └── quizzes/            # Quiz exercises
└── solutions/
    ├── XX_topic_name/      # Matching directory structure
    │   ├── exercise1.rs    # Solution files
    │   └── exercise2.rs
    └── quizzes/
```

## Step-by-Step Guide

### 1. **Choose a Topic Number and Name**

Topics are numbered sequentially (e.g., `00_intro`, `01_variables`, `23_conversions`). Choose the next available number or insert between existing topics.

### 2. **Create Directory Structure**

```bash
# Create exercise directory
mkdir -p exercises/24_your_topic

# Create solution directory
mkdir -p solutions/24_your_topic
```

### 3. **Create Exercise Files**

Create your exercise file in `exercises/24_your_topic/your_exercise1.rs`:

```rust
fn main() {
    // TODO: Add your exercise instructions here
    let x = 5;
    
    println!("x has the value {x}");
}
```

**Exercise File Guidelines:**
- Use `// TODO:` comments to guide learners
- Include [main()](cci:1://file:///home/kennygu/trunk/rustlings/exercises/01_variables/variables1.rs:0:0-5:1) function or test modules
- Leave intentional gaps for learners to fill
- Keep exercises focused on one concept

### 4. **Create Solution Files**

Create the corresponding solution in `solutions/24_your_topic/your_exercise1.rs`:

```rust
fn main() {
    // Explanation of the solution
    let x = 5;
    
    println!("x has the value {x}");
}
```

**Solution File Guidelines:**
- Provide clear explanatory comments
- Show best practices
- Match the exercise filename exactly

### 5. **Add Topic README (Optional but Recommended)**

Create `exercises/24_your_topic/README.md`:

```markdown
# Your Topic Name

Brief explanation of the concept being taught.

## Further information

- [Relevant Rust Book Chapter](https://doc.rust-lang.org/book/...)
- Additional resources
```

### 6. **Register Exercises in Cargo.toml**

This is the **most critical step**. Add entries to the `bin` array in [Cargo.toml](cci:7://file:///home/kennygu/trunk/rustlings/Cargo.toml:0:0-0:0):

```toml
bin = [
  # ... existing entries ...
  
  # Your new exercises
  { name = "your_exercise1", path = "exercises/24_your_topic/your_exercise1.rs" },
  { name = "your_exercise1_sol", path = "solutions/24_your_topic/your_exercise1.rs" },
  { name = "your_exercise2", path = "exercises/24_your_topic/your_exercise2.rs" },
  { name = "your_exercise2_sol", path = "solutions/24_your_topic/your_exercise2.rs" },
]
```

**Important Notes:**
- Exercise name: `exercise_name`
- Solution name: `exercise_name_sol` (must have `_sol` suffix)
- Paths must be exact and relative to the repository root
- Order matters - exercises appear in the order listed

### 7. **Update Exercise Mapping (Optional)**

If your topic corresponds to a Rust Book chapter, update [exercises/README.md](cci:7://file:///home/kennygu/trunk/rustlings/exercises/README.md:0:0-0:0):

```markdown
| your_topic             | §X.Y                |
```

### 8. **Test Your Exercises**

```bash
# Test the exercise compiles (should fail if incomplete)
cargo run --bin your_exercise1

# Test the solution compiles and runs
cargo run --bin your_exercise1_sol

# Verify all exercises still work
cargo build --bins
```

## Example: Adding a Custom "Pattern Matching" Exercise

### 1. Create directories:
```bash
mkdir -p exercises/24_pattern_matching
mkdir -p solutions/24_pattern_matching
```

### 2. Create `exercises/24_pattern_matching/patterns1.rs`:
```rust
fn main() {
    let number = 7;
    
    // TODO: Use a match expression to print whether the number is
    // "small" (0-5), "medium" (6-10), or "large" (>10)
}
```

### 3. Create `solutions/24_pattern_matching/patterns1.rs`:
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

### 4. Create `exercises/24_pattern_matching/README.md`:
```markdown
# Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to compare values against patterns.

## Further information

- [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
```

### 5. Add to [Cargo.toml](cci:7://file:///home/kennygu/trunk/rustlings/Cargo.toml:0:0-0:0):
```toml
  { name = "patterns1", path = "exercises/24_pattern_matching/patterns1.rs" },
  { name = "patterns1_sol", path = "solutions/24_pattern_matching/patterns1.rs" },
```

## Key Points to Remember

1. **Naming Convention**: Exercise binaries don't include the directory prefix (use `patterns1`, not `24_pattern_matching_patterns1`)

2. **Solution Suffix**: Solutions must end with `_sol` in the binary name

3. **Directory Numbering**: Use two-digit prefixes (e.g., `01_`, `24_`) for proper sorting

4. **File Synchronization**: Every exercise must have a corresponding solution with the exact same filename

5. **Cargo.toml is Required**: Exercises won't be recognized by rustlings without being registered in [Cargo.toml](cci:7://file:///home/kennygu/trunk/rustlings/Cargo.toml:0:0-0:0)

6. **Testing**: Always test both the exercise (should have compilation errors or incomplete code) and solution (should compile and run successfully)

This structure allows rustlings to:
- Track student progress
- Provide hints and solutions
- Run exercises in a specific order
- Verify completeness