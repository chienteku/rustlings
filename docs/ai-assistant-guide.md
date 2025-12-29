# AI Assistant Guide: Creating Custom Rustlings Exercises

This guide is designed for AI assistants (Gemini, Claude, ChatGPT, etc.) to help users create custom rustlings exercises through an interactive, collaborative process.

## Overview

This workflow guides you through helping users transform their Rust concepts or code snippets into fully-integrated rustlings exercises. The process involves analysis, discussion, implementation, and version control.

---

## Workflow Steps

### Step 1: Receive and Analyze User Input

When a user provides a concept or code snippet, perform the following analysis:

#### 1.1 Understand the Input

**User might provide:**
- A Rust concept (e.g., "I want to teach async/await")
- A code snippet (e.g., a function demonstrating a pattern)
- A learning objective (e.g., "Students should understand trait objects")
- A problem they want students to solve

#### 1.2 Analyze the Concept

Determine the following:

**A. Core Rust Concept**
- What fundamental Rust concept does this teach?
- What prerequisite knowledge is required?
- What's the learning difficulty level?

**B. Category Classification**

Check if it fits existing categories:

```
Existing Categories:
00_intro          - Introduction to rustlings
01_variables      - Variable bindings, mutability, shadowing
02_functions      - Function definitions, parameters, return values
03_if             - If expressions, conditional logic
04_primitive_types - Booleans, chars, arrays, tuples, slices
05_vecs           - Vectors, dynamic arrays
06_move_semantics - Ownership, moves, borrows
07_structs        - Struct definitions, methods
08_enums          - Enum definitions, pattern matching basics
09_strings        - String vs &str, string operations
10_modules        - Module system, visibility
11_hashmaps       - HashMap operations
12_options        - Option<T> handling
13_error_handling - Result<T, E>, error propagation
14_generics       - Generic types and functions
15_traits         - Trait definitions and implementations
16_lifetimes      - Lifetime annotations
17_tests          - Writing tests
18_iterators      - Iterator trait, iterator methods
19_smart_pointers - Box, Rc, Arc, Cow
20_threads        - Threading, concurrency
21_macros         - Macro definitions and usage
22_clippy         - Clippy lints and best practices
23_conversions    - Type conversions (From, Into, TryFrom, etc.)
```

**C. Categorization Decision**

Determine one of the following:

1. **Fits Existing Category**: The concept belongs to an existing category
   - Example: "Teaching range patterns" → fits `08_enums` or create `24_pattern_matching`
   
2. **Needs New Category**: The concept is substantial enough for a new category
   - Example: "Teaching async/await" → needs `XX_async_await`
   
3. **Extends Existing Category**: Adds advanced topics to existing category
   - Example: "Advanced iterator combinators" → extends `18_iterators`

#### 1.3 Analysis Template

Present your analysis to the user:

```markdown
## Analysis of Your Concept

**Concept**: [Brief description]

**Core Rust Topics**:
- [Topic 1]
- [Topic 2]
- [Topic 3]

**Difficulty Level**: [Beginner/Intermediate/Advanced]

**Prerequisites**: 
- [Prerequisite 1]
- [Prerequisite 2]

**Category Recommendation**:
[Fits existing category X / Needs new category / Extends category Y]

**Reasoning**: [Explain why this categorization makes sense]
```

---

### Step 2: Discuss Exercise Design with User

Engage in a collaborative discussion to design the exercise(s).

#### 2.1 Propose Exercise Structure

**Template for Discussion:**

```markdown
## Proposed Exercise Design

### Exercise Name
`[category_name][number].rs` (e.g., `pattern_matching1.rs`)

### Learning Objectives
After completing this exercise, students will be able to:
- [Objective 1]
- [Objective 2]
- [Objective 3]

### Exercise Concept
[Brief description of what the exercise asks students to do]

### Starter Code Structure
```rust
// TODO: [Clear instruction]

fn main() {
    // Starter code that students will complete
}

// Optional: Tests
#[cfg(test)]
mod tests {
    // Test cases
}
```

### Solution Approach
[Brief description of the solution without giving it away]

### Hint Strategy
**Hint 1 (Conceptual)**: [High-level guidance]
**Hint 2 (Specific)**: [More specific pointer]
**Hint 3 (Resource)**: [Link to documentation]

### Difficulty Progression
[If multiple exercises, explain how difficulty increases]
```

#### 2.2 Discussion Points

Ask the user:

1. **Scope**: "Should this be a single exercise or a series (e.g., pattern1, pattern2, pattern3)?"

2. **Difficulty**: "What difficulty level are you targeting? Should students struggle a bit or should it be straightforward?"

3. **Testing**: "Should the exercise include tests, or just compile successfully?"

4. **Context**: "Should we provide a real-world scenario or keep it abstract?"

5. **Hints**: "How much guidance should the hints provide? Step-by-step or conceptual?"

#### 2.3 Iterate Based on Feedback

- Adjust the exercise design based on user feedback
- Refine learning objectives
- Modify difficulty level
- Add or remove test cases
- Improve hint quality

#### 2.4 Present Final Proposal

Once the design is refined, present a complete proposal:

```markdown
## Final Exercise Proposal

### Category
[New category: XX_category_name] OR [Existing category: YY_existing_name]

### Exercise(s)
1. **[exercise1_name]**
   - Learning objective: [...]
   - Starter code: [brief description]
   - Tests: [Yes/No]
   
2. **[exercise2_name]** (if applicable)
   - Learning objective: [...]
   - Starter code: [brief description]
   - Tests: [Yes/No]

### Complete Example

**Exercise File**: `exercises/XX_category/exercise1.rs`
```rust
[Show complete starter code with TODOs]
```

**Solution File**: `solutions/XX_category/exercise1.rs`
```rust
[Show complete solution]
```

**Hint**:
```
[Show the hint text that will go in info.toml]
```

**README.md**:
```markdown
[Show the topic README content]
```

### info.toml Entry
```toml
[[exercises]]
name = "exercise1"
dir = "XX_category"
test = [true/false]
hint = """[hint text]"""
```

---

**Does this look good? Should I proceed with implementation?**
```

---

### Step 3: Implement After User Approval

Once the user approves the design, follow the implementation steps from `docs/adding-custom-exercises.md`.

#### 3.1 Implementation Checklist

Present this checklist and execute each step:

```markdown
## Implementation Plan

I will now:
- [ ] Create directory structure
- [ ] Create exercise file(s) with TODO comments
- [ ] Create solution file(s)
- [ ] Create README.md for the topic
- [ ] Add exercise(s) to info.toml
- [ ] Run `rustlings dev update` to sync Cargo.toml
- [ ] Run `rustlings dev check` to validate
- [ ] Test with `rustlings run`, `rustlings hint`
- [ ] Verify interactive mode works

Proceeding with implementation...
```

#### 3.2 Create Files

**A. Directory Structure**

```bash
mkdir -p exercises/XX_category_name
mkdir -p solutions/XX_category_name
```

**B. Exercise File(s)**

Create each exercise file with:
- Clear `// TODO:` comments (REQUIRED)
- Starter code that compiles (if possible)
- Test cases (if applicable)
- Helpful comments explaining context

**C. Solution File(s)**

Create matching solution files with:
- Complete, working code
- Explanatory comments
- Best practices demonstrated

**D. README.md**

Create topic README with:
- Topic introduction
- Key concepts list
- Learning resources
- Links to Rust documentation

#### 3.3 Update Configuration

**A. Add to info.toml**

```toml
[[exercises]]
name = "exercise_name"
dir = "XX_category_name"
test = false  # or true if has #[test] annotations
skip_check_unsolved = true  # if working in solved repository
hint = """
[Multi-line hint with:
- Conceptual guidance
- Specific pointers
- Resource links]
"""
```

**B. Update Cargo.toml**

```bash
rustlings dev update
```

#### 3.4 Validation

Run validation commands and report results:

```bash
# Validate configuration
rustlings dev check

# Test specific exercise
rustlings run exercise_name

# Test hint
rustlings hint exercise_name

# Test interactive mode
rustlings
```

**Report any errors and fix them before proceeding.**

---

### Step 4: Git Commit

After successful validation, commit the changes.

#### 4.1 Review Changes

Show the user what will be committed:

```bash
git status
```

Present a summary:
```markdown
## Changes to Commit

### New Files:
- exercises/XX_category/exercise1.rs
- solutions/XX_category/exercise1.rs
- exercises/XX_category/README.md

### Modified Files:
- info.toml (added X exercise(s))
- Cargo.toml (added X binaries)

### Validation Results:
✅ rustlings dev check passed
✅ All exercises compile
✅ Hints display correctly
✅ Interactive mode works
```

#### 4.2 Create Commit Message

Generate a descriptive commit message:

```
Add [category_name] exercise(s) with full rustlings integration

- Created [X] new exercise(s) in XX_category_name/
- Added [exercise_name] teaching [concept]
- Updated info.toml with exercise metadata and hints
- Updated Cargo.toml via rustlings dev update
- All exercises pass rustlings dev check validation

The exercise(s) teach [brief description of what students learn]
and are fully integrated with rustlings CLI.
```

#### 4.3 Execute Commit

```bash
git add .
git commit -m "[commit message]"
```

Confirm success and show commit hash.

---

## Example Interaction Flow

Here's a complete example of how to guide a user:

### User Input
> "I want to teach students about the `?` operator for error propagation"

### Step 1: Analysis

```markdown
## Analysis of Your Concept

**Concept**: Error propagation using the `?` operator

**Core Rust Topics**:
- Result<T, E> type
- Error propagation
- The `?` operator syntax
- Function return types

**Difficulty Level**: Intermediate

**Prerequisites**: 
- Understanding of Result<T, E>
- Basic error handling with match
- Function return types

**Category Recommendation**: 
Fits existing category `13_error_handling`

**Reasoning**: The `?` operator is a core error handling feature in Rust. 
The existing error_handling category (errors1-6) covers Result basics and 
error types. Adding an exercise specifically about `?` would complement 
the existing exercises nicely. We could add it as `errors7.rs`.

However, I notice errors3.rs already mentions the `?` operator. Would you 
like to:
1. Enhance errors3.rs to focus more on `?`
2. Create a new errors7.rs with advanced `?` usage
3. Create a new category for advanced error handling patterns
```

### Step 2: Discussion

```markdown
## Proposed Exercise Design

### Exercise Name
`errors7.rs` (extending existing error_handling category)

### Learning Objectives
After completing this exercise, students will be able to:
- Use the `?` operator to propagate errors
- Understand when `?` can and cannot be used
- Write functions that return Result with `?`
- Chain multiple operations that might fail

### Exercise Concept
Students will complete a file reading function that uses `?` to propagate 
errors from multiple operations (opening file, reading content, parsing).

### Starter Code Structure
```rust
use std::fs;
use std::io;

// TODO: Complete this function using the ? operator
// Read a file and parse its first line as a number
fn read_number_from_file(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // TODO: Open the file (use ?)
    // TODO: Read the content (use ?)
    // TODO: Parse the first line (use ?)
}

fn main() {
    match read_number_from_file("number.txt") {
        Ok(n) => println!("Number: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_read_number() {
        // Test implementation
    }
}
```

### Hint Strategy
**Conceptual**: The `?` operator can only be used in functions that return 
Result or Option. It automatically propagates errors up the call stack.

**Specific**: Use `fs::read_to_string(path)?` to read the file. The `?` 
will return early if there's an error.

**Resource**: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

---

**Questions**:
1. Should we include file creation in the test setup?
2. Do you want to cover `?` with Option as well, or just Result?
3. Should this be a single exercise or split into errors7 and errors8?
```

### Step 3: Implementation (After Approval)

```markdown
## Implementation Plan

Based on your approval, I will:
- [x] Create exercises/13_error_handling/errors7.rs
- [x] Create solutions/13_error_handling/errors7.rs
- [x] Add entry to info.toml
- [x] Run rustlings dev update
- [x] Run rustlings dev check
- [x] Test all commands

Proceeding...
```

[Execute implementation steps]

### Step 4: Commit

```markdown
## Changes to Commit

### New Files:
- exercises/13_error_handling/errors7.rs
- solutions/13_error_handling/errors7.rs

### Modified Files:
- info.toml (added errors7 exercise)
- Cargo.toml (added errors7 and errors7_sol binaries)

### Validation Results:
✅ rustlings dev check passed (97/97 exercises)
✅ rustlings run errors7 works
✅ rustlings hint errors7 displays correctly
✅ Interactive mode shows exercise

Committing with message:
"Add errors7 exercise teaching the ? operator for error propagation"
```

---

## Best Practices for AI Assistants

### 1. Always Analyze First
- Don't jump straight to implementation
- Understand the user's intent and learning goals
- Consider prerequisite knowledge

### 2. Collaborate, Don't Dictate
- Present options, not mandates
- Ask questions to clarify requirements
- Iterate based on feedback

### 3. Provide Context
- Explain why you're suggesting a particular approach
- Reference existing exercises for consistency
- Show how the new exercise fits into the learning path

### 4. Validate Thoroughly
- Always run `rustlings dev check`
- Test all rustlings commands
- Verify hints are helpful but not revealing

### 5. Document Clearly
- Use clear TODO comments
- Write helpful hints with resources
- Create informative README files

### 6. Follow Conventions
- Match existing naming patterns
- Use consistent code style
- Follow rustlings best practices

### 7. Handle Errors Gracefully
- If validation fails, explain why
- Provide clear steps to fix issues
- Don't proceed to commit if errors exist

---

## Common Scenarios

### Scenario 1: User Provides Abstract Concept

**User**: "I want to teach trait objects"

**Your Response**:
1. Analyze: This is an advanced topic related to traits and dynamic dispatch
2. Check: Does it fit in `15_traits` or need new category?
3. Discuss: Complexity level, should it be multiple exercises?
4. Propose: Concrete exercise ideas with code examples
5. Iterate: Refine based on feedback

### Scenario 2: User Provides Code Snippet

**User**: [Pastes a function using async/await]

**Your Response**:
1. Analyze the code: What concepts does it demonstrate?
2. Identify: This needs a new category (async not in current exercises)
3. Discuss: How to simplify for learning? What prerequisites?
4. Propose: Series of exercises building up to this concept
5. Implement: After approval

### Scenario 3: User Wants to Extend Existing Category

**User**: "Can we add more iterator exercises?"

**Your Response**:
1. Review: Check existing iterator exercises (iterators1-5)
2. Identify gaps: What's not covered? (e.g., custom iterators, complex chains)
3. Propose: New exercises filling those gaps
4. Discuss: Difficulty progression
5. Implement: Add as iterators6, iterators7, etc.

---

## Error Handling

### Common Errors and Solutions

**Error**: "Didn't find any `// TODO` comment"
```
Solution: Add at least one // TODO: comment to the exercise file
```

**Error**: "The exercise X is already solved"
```
Solution: Add skip_check_unsolved = true to info.toml
```

**Error**: "has #[test] but test = false"
```
Solution: Remove test = false from info.toml
```

**Error**: "Cargo.toml is outdated"
```
Solution: Run rustlings dev update
```

### Validation Checklist

Before committing, ensure:
- [ ] All exercise files have `// TODO:` comments
- [ ] Solution files exist and compile
- [ ] `info.toml` entries are correct
- [ ] `test = false` only if no `#[test]` annotations
- [ ] Hints are helpful and include resources
- [ ] `rustlings dev check` passes
- [ ] `rustlings run <exercise>` works
- [ ] `rustlings hint <exercise>` displays correctly
- [ ] Interactive mode shows exercise

---

## Summary

As an AI assistant, your role is to:

1. **Analyze** user input to understand the Rust concept
2. **Categorize** the concept appropriately
3. **Discuss** exercise design collaboratively
4. **Implement** following `docs/adding-custom-exercises.md`
5. **Validate** thoroughly before committing
6. **Commit** with descriptive messages

Always prioritize:
- Clear communication
- Educational value
- Rustlings conventions
- Thorough validation
- User collaboration

Remember: The goal is to create high-quality learning exercises that help students understand Rust concepts effectively!
