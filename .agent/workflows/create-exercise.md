---
description: Create a new custom Rustlings exercise from a concept or code snippet
---

1. **Load Context**: Read the AI Assistant Guide at `docs/ai-assistant-guide.md` to understand the required workflow and best practices.

2. **Analyze Input**: 
   - If the user provided a concept or code, analyze it using the "Analysis Template" from the guide.
   - If no input was provided, ask the user for the concept they want to teach.

3. **Discuss Design**:
   - Propose a categorisation (existing vs. new).
   - Outline the exercise structure, learning objectives, and hints.
   - Wait for user approval.

4. **Implement**:
   - Once approved, follow the implementation steps in the guide (referencing `docs/adding-custom-exercises.md`).
   - Create the exercise `.rs` file, solution `.rs` file, and `README.md`.
   - Update `info.toml` and `Cargo.toml`.

5. **Validate**:
   - Run `rustlings dev check`.
   - Run `rustlings dev update`.
   - Verify with `rustlings run <exercise>`.

6. **Commit**:
   - Git commit the changes using the format specified in the guide.
