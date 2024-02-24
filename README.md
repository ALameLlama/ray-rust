# ray-rust

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
ray-rust = "0.1"
```
