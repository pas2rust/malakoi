# Malakoi

[![Crates.io](https://img.shields.io/crates/v/malakoi.svg)](https://crates.io/crates/malakoi)
[![Docs.rs](https://docs.rs/malakoi/badge.svg)](https://docs.rs/malakoi)
[![License](https://img.shields.io/crates/l/malakoi.svg)](https://github.com/pas2rust/malakoi/blob/main/LICENSE)
![GitHub top language](https://img.shields.io/github/languages/top/pas2rust/malakoi?color=orange&logo=rust&style=flat&logoColor=white)
![GitHub stars](https://img.shields.io/github/stars/pas2rust/malakoi?color=success&style=flat&logo=github)
![GitHub forks](https://img.shields.io/github/forks/pas2rust/malakoi?color=orange&logo=Furry%20Network&style=flat&logoColor=white)
![Tests](https://raw.githubusercontent.com/pas2rust/badges/main/malakoi-tests.svg)
![Crates.io downloads](https://img.shields.io/crates/d/malakoi.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/pas2rust/malakoi?color=ff69b4&label=update&logo=git&style=flat&logoColor=white)

---

Malakoi is a **Rust crate for advanced procedural macros and utility components**, designed to enhance productivity and ergonomics in Rust development. It focuses on **math helpers, macro-driven workflows, and code generation** for repetitive patterns.

## Features

- `Math` derive for numeric structs:
  - Generate `approach_*` methods to smoothly move numeric fields toward a target with a max delta.
- `Builder` integrations (via `kenzu::Builder`) for ergonomic initialization.
- Easy integration with structs of **any numeric type** (`usize`, `i32`, `f64`, etc.).
- Fully tested examples and doc-tests for reliable usage.
- Zero dependencies outside standard Rust ecosystem + optional feature helpers.

## Installation

Add Malakoi to your `Cargo.toml`:

```bash
cargo add malakoi
```

```rust
use kenzu::Builder;
use malakoi::Math;

#[derive(Debug, Builder, Math, Clone, Default)]
pub struct CalcStruct {
    pub usize: usize,
    pub u8: u8,
    pub u16: u16,
    pub u32: u32,
    pub u64: u64,
    pub u128: u128,
    pub isize: isize,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub i128: i128,
    pub f64: f64,
    pub f32: f32,
}

#[test]
fn approach_usize() {
    let mut calc_struct = CalcStruct::new().usize::<usize>(10).build().unwrap();
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 16);
    calc_struct.approach_usize(20, 6);
    assert_eq!(calc_struct.usize, 20);
}


#[test]
fn discount_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .build()
        .unwrap();
    calc_struct.discount_usize(20);
    assert_eq!(calc_struct.usize, 8);
}

#[test]
fn div_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .build()
        .unwrap();
    calc_struct.div_usize(5);

    assert_eq!(calc_struct.usize, 2);
}

#[test]
fn inflate_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(10)
        .build()
        .unwrap();
    calc_struct.inflate_usize(20);
    assert_eq!(calc_struct.usize, 12);
}

#[test]
fn mul_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(1)
        .build()
        .unwrap();
    calc_struct.mul_usize(5);
    assert_eq!(calc_struct.usize, 5);
}

#[test]
fn sub_usize() {
    let mut calc_struct = CalcStruct::new()
        .usize::<usize>(6)
        .build()
        .unwrap();
    calc_struct.sub_usize(5);
    assert_eq!(calc_struct.usize, 1);
}

#[test]
fn sum_usize() {
    let mut calc_struct = CalcStruct::new().usize::<usize>(1).build().unwrap();
    calc_struct.sum_usize(5);
    assert_eq!(calc_struct.usize, 6);
}

```

# ❤️ Donate

[![Monero](https://img.shields.io/badge/88NKLkhZf1nTVpaSU6vwG6dwBwb9tFVSM8Lpj3YqdL1PMt8Gm7opV7aUnMYBaAC9Y6a4kfDc3fLGoMVqeSJKNphyLpLdEvC-FF6600?style=flat&logo=monero&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-monero-donate.png)
[![Bitcoin](https://img.shields.io/badge/bc1qnlayyh84e9u5pd4m9g9sf4c5zdzswvkmudmdu5-EAB300?style=flat&logo=bitcoin&logoColor=white)](https://github.com/pas2rust/pas2rust/blob/main/pas-bitcoin-donate.png)