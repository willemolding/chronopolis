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

or in release mode for speed

```shell
cargo run -r
```

## Adding a new clockface

Look in the [faces](./src/faces/) directory for examples. You want to add a new file that exports a struct that implements the `ClockFace` trait

```rust
pub trait ClockFace: Debug {
    /// Shown in the switcher / logs.
    fn name(&self) -> &str;

    /// Per-frame state update. Default no-op for purely stateless faces.
    fn update(&mut self, _app: &App, _ctx: &ClockContext) {}

    /// Draw the face. `draw` is pre-made and flushed for you — just draw.
    fn view(&self, app: &App, ctx: &ClockContext, draw: &Draw);
}
```

and add the module in [faces/mod.rs](./src/faces/mod.rs) e.g.

```rust
pub mod yourface;

pub fn all() -> Vec<Box<dyn ClockFace>> {
    vec![
        ...
        Box::new(yourface::YourFace::new()),
    ]
}
```

You can check out the [Nannou drawing guide](https://guide.nannou.cc/tutorials/draw/drawing-2d-shapes) for help in drawing your face.
