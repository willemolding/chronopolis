# Chronopolis

Chronopolis is a clock tower art installation with crowd sourced clock faces.

It might tell the time, it might not. It is entirely up to you!

## Prerequisites

- [Install Rust](https://rust-lang.org/tools/install/)

Optional

- Install the [rust-analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) if using VSCode 

## Running

```shell
cargo run
```

## Adding a new clockface

Look in the [faces](./src/faces/) directory for examples. You want to add a new file that exports a single function

```rust
pub fn render(app: &App, model: &Model, frame: Frame) {
}
```

and add the module in [faces/mod.rs](./src/faces/mod.rs) e.g.

```rust
pub mod yourface;
```

You can check out the [Nannou drawing guide](https://guide.nannou.cc/tutorials/draw/drawing-2d-shapes) for help in drawing your face.

To install the face replace the line in [main.rs](./src/main.rs)

```rust
faces::analog::render(app, model, frame);
```

with the function for your new face!

```rust
faces::yourface::render(app, model, frame);
```
