<div align="center">

# 🦀 The Rust Training Arc
### Fighting the borrow checker instead of getting a summer internship.

![Language](https://img.shields.io/badge/Language-Rust-dea584?style=for-the-badge&logo=rust&logoColor=black)
![Environment](https://img.shields.io/badge/Env-no__std-ff5e00?style=for-the-badge&logo=rust&logoColor=white)
![Target](https://img.shields.io/badge/Target-AArch64-00599C?style=for-the-badge&logo=arm&logoColor=white)

</div>

---

## ⚡ The Manifesto
This repository is my dedicated sandbox for mastering Rust systems programming from the absolute ground up. Since LinkedIn is currently a toxic wasteland of internship announcements, I decided to spend my summer proving I can write zero-allocation hardware drivers and bare-metal systems. 

The ultimate goal? Get good enough to catch the attention of Apple, NVIDIA, or AMD.

> "Safety without a garbage collector. Pain without compromise."

## 💎 Core Directives

| Directive | Description |
| :--- | :--- |
| **Zero Warnings** | Warnings are just errors that haven't hit the gym yet. Absolute silence is required. |
| **Clippy is God** | `cargo clippy` is my micro-managing boss. If Clippy complains, the code doesn't commit. |
| **Quarantined `unsafe`** | We only take off the memory-safe training wheels when we *explicitly* want to crash the bike. |
| **Bare-Metal Focus** | We prefer raw byte manipulation and performance over heavy, bloated abstractions. |

## 🛠️ The Training Blueprint (Modules)

- [ ] **Phase 1: Day-One Basics** (Wrestling the compiler, crying over Ownership)
  - `01_strict_counter`: Simulated fitness tracker to learn that variables are immutable by default.
  - `02_cli_calculator`: Pattern matching via `match` statements because `if/else` is for cowards.
  - `03_system_logs`: Custom `structs` and `enums` to categorize system states.
- [ ] **Phase 2: Systems Fundamentals** (Moving bytes, avoiding the heap like the plague)
  - `04_wav_parser`: Reading raw `.wav` byte streams and dealing with endianness.
  - `05_ring_buffer`: Const Generics circular queue for zero heap allocation.
  - `06_gap_buffer`: Localized `unsafe` blocks to shift memory slices efficiently.
- [ ] **Phase 3: The Gateway to Bare-Metal** (Pretending we are an OS)
  - `07_virtual_registers`: Safe abstractions over hardware MMIO (`read_volatile` / `write_volatile`).
- [ ] **Phase 4: Custom OS Ready** (Actually booting on silicon)
  - AArch64 bare-metal audio sequencer (To be continued...)

## 🚀 Boot Sequence (Environment)

You need `rustup`, `cargo`, and a high tolerance for compiler error messages. Engineered on Arch Linux (because of course it is).

```bash
# Clone the repository
git clone https://github.com/Shrimanpro/rust_training_arc.git
cd rust_training_arc

# Ignite a module and prepare to be judged by rustc
cd 01_strict_counter
cargo run
```

## 📜 License
MIT License.<div align="center">
*"I am once again asking the compiler to let my code run."*
</div>
