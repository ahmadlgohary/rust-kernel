# Minimal x86_64 Rust Kernel

`What I cannot create, I do not understand` — Richard Feynman

A small x86_64 kernel written in **Rust**.

This is mainly a learning project serving as a playground for experimenting with low-level OS concepts.

---

## Features

* Bare-metal `no_std` Rust kernel
* Heap memory
* Interrupt handling
* Async input handling
* Fault handling
* Paging

---

## Status

The current implementation closely follows the referenced tutorial and acts as a baseline for further work.

---

## Planned

**Short term**
- Optimized linked-list allocator
- Preemptive scheduler (timer interrupt–driven)
- Context switching
- Timer-based multitasking

**Long term**
- Basic CLI
- Simple Space Invaders–style game


---

## Build & Run

```bash
cargo run
```

## Acknowledgements

Based on [Philipp Oppermann's *Writing an OS in Rust*](https://os.phil-opp.com/).
