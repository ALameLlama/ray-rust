# ray-rust

<p align="center">
<a href="https://github.com/ALameLlama/ray-rust/actions/workflows/publish.yml"><img src="https://img.shields.io/github/actions/workflow/status/ALameLlama/ray-rust/.github/workflows/publish.yml" alt="Build + Test Status"></a>
<a href="https://crates.io/crates/ray-rust"><img src="https://img.shields.io/crates/v/ray-rust" alt="Latest Stable Version"></a>
<a href="https://crates.io/crates/ray-rust"><img alt="Crates.io Total Downloads" src="https://img.shields.io/crates/d/ray-rust"></a>
<a href="https://github.com/ALameLlama/ray-rust/blob/master/LICENSE"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License"></a>
</p>

This is currently WIP and still missing features but has basic ray debugging.

## Examples

```rust
use ray_rust::*

fn main() {
    ray!("Hello World");

    ray!("Hello World!").color("green");

    ray!().html("<strong>Hello World! 🦀</strong>");
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ray-rust = "0.1.5"
```

## Features

```toml
ray-rust = { version = "0.1.5", features = ["with_tokio"] }
```
