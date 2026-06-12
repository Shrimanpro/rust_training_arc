# 02: CLI Calculator
**Phase:** Phase 1 (Basics)

## 🎯 The Objective
Meet the borrow checker and learn how Rust handles errors without crashing. This teaches you the mental model of how Rust cleans up memory without needing a garbage collector like Java or Go.

## 🛠️ Implementation Specs

- [ ] Import `std::io` and use `std::io::stdin().read_line()` to grab user input as a String.
- [ ] Use `.split_whitespace()` to isolate the numbers and the operator into separate variables.
- [ ] Parse the string numbers to `f32`. **Constraint:** You MUST use a `match` statement to handle the `Result`. Do not use `.unwrap()`. If parsing fails, print a warning and continue.
- [ ] Use a `match` statement on the operator string (`"+"`, `"-"`, `"*"`, `"/"`) to compute the final value.
- [ ] Wrap the execution in a loop that listens continuously until the user types `"quit"`.
