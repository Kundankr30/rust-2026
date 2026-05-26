# Advancing with Rust (2026)

Hands-on repository for the **Advancing with Rust** internship at **Silicon** (2026).

This repo collects the code written while working through the Rust fundamentals,
one numbered module at a time. Each module is self-contained so it can be built
and run on its own.

## Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (`rustc` + `cargo`), edition 2024
  - Built against `rustc 1.93.0` / `cargo 1.93.0`

Check your install:

```sh
rustc --version
cargo --version
```

## Modules

| # | Module | What it covers |
|---|--------|----------------|
| 00 | [`00-cargo-and-rustc`](./00-cargo-and-rustc) | Compiling directly with `rustc` vs. managing a project with Cargo; crates and `Cargo.toml` |
| 01 | [`01-variables-and-data-types`](./01-variables-and-data-types) | `let`/`mut`, constants, scalar types (integers, floats, bool, char), and `as` type casting |

### 00: Cargo and rustc

- `main.rs`: a standalone file compiled directly with `rustc`.
- `silicon/`: the first Cargo project (`cargo new`).

### 01: Variables and data types

- `vars/`: Cargo project exploring immutability and `mut`, the `const PI`,
  integer/float/bool/char types, byte representation of characters, and numeric
  casting with `as` (including overflow wrap-around and float precision).

## Running the code

**Cargo projects** (e.g. `silicon`, `vars`). `cd` into the project directory:

```sh
cd 01-variables-and-data-types/vars
cargo run
```

**Standalone files** compiled with `rustc`:

```sh
cd 00-cargo-and-rustc
rustc main.rs && ./main
```

## Layout

```
rust-2026/
├── 00-cargo-and-rustc/
│   ├── main.rs              # compiled with rustc directly
│   └── silicon/             # cargo project
└── 01-variables-and-data-types/
    └── vars/                # cargo project
```
