# 01: Strict Counter

**Phase:** Phase 1 (Basics)

## 🎯 The Objective

Break the C habit of mutating variables anywhere. Learn how Rust enforces explicit type safety and immutability by default. In an OS, accidentally modifying a variable that holds a hardware memory address can crash the entire system. Rust forces this discipline on day one.

## 🛠️ Implementation Specs

- [x] Initialize a `mut` steps variable and an immutable `daily_goal`.

- [x] Create a `loop` (or `while` loop) that increments your steps. (Using a function)

- [ ] Write an `if` statement using the `break` keyword to exit the loop when steps >= goal.

- [ ] Calculate the completion percentage. **Constraint:** You must cast your integers using `as f32` before dividing, otherwise the compiler will reject it.

- [ ] Print the progress to `stdout` on every iteration using `println!()`.
