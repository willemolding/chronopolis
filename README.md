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

## Running it on the tower

[firmware/](./firmware/) builds a bootable SD card image for a Raspberry Pi Zero W that powers on straight into the clock — no desktop, no login, no operator. It is a [Buildroot](https://buildroot.org) br2-external tree plus a pinned build container, so all you need on your machine is Docker:

```shell
make -C firmware config    # build the container and the Buildroot config
make -C firmware image     # ~1-3h the first time; incremental after that
```

The image lands at `firmware/artifacts/sdcard.img`. See
[firmware/README.md](./firmware/README.md) for flashing, tuning and how to get a serial console into a running installation.

## Adding a new clockface

Look in the [faces](./src/faces/) directory for examples. You want to add a new file that exports a struct that implements the `ClockFace` trait

```rust
pub trait ClockFace: Debug {
    /// Shown in the switcher / logs.
    fn name(&self) -> &str;

    /// Per-frame state update. Default no-op for purely stateless faces.
    fn update(&mut self, _ctx: &ClockContext) {}

    /// Draw the face with macroquad calls. The origin is the centre of the
    /// window and `+y` points down, so positive angles turn clockwise.
    fn view(&self, ctx: &ClockContext);
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

### Drawing

Start every face with `use crate::prelude::*;`, which pulls in macroquad plus
the bits of this project you need. Drawing is plain
[macroquad](https://docs.rs/macroquad/latest/macroquad/) — `draw_circle`,
`draw_line`, `draw_texture_ex`

Two things are set up for you:

- **The origin is the centre of the window**, one unit per pixel, with `+y`
  pointing down. That means a positive rotation is clockwise and `ctx.radius` is the radius of the circular canvas you have to
  play with.
- **Angles** on `ClockContext` (`hour_angle`, `min_angle`, `sec_angle`) are in
  radians, measured clockwise from 12 o'clock.

[draw.rs](./src/draw.rs) adds the few helpers macroquad is missing:

### Artwork

Drop images under [assets/](./assets/) and read them back with
`ctx.texture("yourface/bg")` — the key is the path below `assets/` without the
file extension. `ctx.canvas()` gives the largest square that fits in the
window, which is the size full-bleed artwork is drawn at.
