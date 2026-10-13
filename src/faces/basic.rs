use crate::prelude::*;

/// One of the four textures a [`BasicFace`] draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Bg,
    Hours,
    Mins,
    Secs,
}

impl Layer {
    /// Background at the bottom, then hours, minutes and seconds on top,
    /// like a normal clock.
    pub const NORMAL: [Layer; 4] = [Layer::Bg, Layer::Hours, Layer::Mins, Layer::Secs];
}

#[derive(Debug)]
pub struct BasicFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
    /// Hands follow the clock continuously instead of ticking.
    sweep: bool,
    name: &'static str,
    /// Draw order, bottom first.
    layers: [Layer; 4],
    bg_key: String,
    hours_key: String,
    mins_key: String,
    secs_key: String,
}

impl BasicFace {
    pub fn new(name: &'static str, path_prefix: &'static str) -> Self {
        Self {
            hour_hand: ClockHand::smooth(),
            min_hand: ClockHand::smooth(),
            sec_hand: ClockHand::bouncy(),
            sweep: false,
            name,
            layers: Layer::NORMAL,
            bg_key: format!("{path_prefix}/bg"),
            hours_key: format!("{path_prefix}/hours"),
            mins_key: format!("{path_prefix}/mins"),
            secs_key: format!("{path_prefix}/secs"),
        }
    }

    /// Override the draw order, listed bottom first. Defaults to [`Layer::NORMAL`].
    pub fn layers(mut self, layers: [Layer; 4]) -> Self {
        self.layers = layers;
        self
    }

    /// Sweep the hands continuously instead of ticking.
    #[allow(dead_code)] // offered to face authors whether or not a face uses it
    pub fn sweeping(mut self) -> Self {
        self.sweep = true;
        self
    }

    fn key(&self, layer: Layer) -> &str {
        match layer {
            Layer::Bg => &self.bg_key,
            Layer::Hours => &self.hours_key,
            Layer::Mins => &self.mins_key,
            Layer::Secs => &self.secs_key,
        }
    }

    /// Sweeping hands read the clock directly rather than going through a
    /// `ClockHand`, so a late frame can't make them stutter.
    fn angle(&self, layer: Layer, ctx: &ClockContext) -> f32 {
        match (layer, self.sweep) {
            (Layer::Bg, _) => 0.0,
            (Layer::Hours, true) => ctx.hour_sweep,
            (Layer::Mins, true) => ctx.min_sweep,
            (Layer::Secs, true) => ctx.sec_sweep,
            (Layer::Hours, false) => self.hour_hand.angle(),
            (Layer::Mins, false) => self.min_hand.angle(),
            (Layer::Secs, false) => self.sec_hand.angle(),
        }
    }
}

impl ClockFace for BasicFace {
    fn name(&self) -> &str {
        self.name
    }

    fn update(&mut self, ctx: &ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn textures(&self) -> Vec<&str> {
        vec![
            &self.bg_key,
            &self.hours_key,
            &self.mins_key,
            &self.secs_key,
        ]
    }

    fn view(&self, ctx: &ClockContext) {
        let size = ctx.canvas();

        // Draw nothing until every layer is resident, rather than pop in.
        let Some(textures) = self
            .layers
            .iter()
            .map(|&layer| ctx.texture(self.key(layer)).map(|t| (layer, t)))
            .collect::<Option<Vec<_>>>()
        else {
            draw_text_centred("Missing textures", Vec2::ZERO, 48, RED);
            return;
        };

        for (layer, texture) in textures {
            draw_texture_centred(texture, Vec2::ZERO, size, self.angle(layer, ctx));
        }
    }
}
