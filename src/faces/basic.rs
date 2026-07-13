use crate::clock_face::ClockFace;
use crate::clock_hand::{ClockHand, Easing};
use nannou::prelude::*;

#[derive(Debug)]
pub struct BasicFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
    name: &'static str,
    path_prefix: &'static str,
}

impl BasicFace {
    pub fn new(name: &'static str, path_prefix: &'static str) -> Self {
        Self {
            hour_hand: ClockHand::new().easing(Easing::Linear).duration(60. * 60.),
            min_hand: ClockHand::new().easing(Easing::Linear).duration(60.),
            sec_hand: ClockHand::sweeping(),
            name,
            path_prefix,
        }
    }
}

impl ClockFace for BasicFace {
    fn name(&self) -> &str {
        self.name
    }

    fn update(&mut self, _app: &App, ctx: &crate::ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, app: &App, ctx: &crate::ClockContext, draw: &Draw) {
        let size = f32::min(app.window_rect().w(), app.window_rect().h());
        let wh = vec2(size, size);

        if let (Some(bg), Some(hours), Some(mins), Some(secs)) = (
            ctx.texture(&format!("{}/bg", self.path_prefix)),
            ctx.texture(&format!("{}/hours", self.path_prefix)),
            ctx.texture(&format!("{}/mins", self.path_prefix)),
            ctx.texture(&format!("{}/secs", self.path_prefix)),
        ) {
            draw.texture(bg).xy(app.window_rect().xy()).wh(wh);
            draw.texture(hours)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.hour_hand.angle());
            draw.texture(mins)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.min_hand.angle());
            draw.texture(secs)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.sec_hand.angle());
        } else {
            draw.text("Missing textures")
                .xy(app.window_rect().xy())
                .color(RED)
                .font_size(48);
        }
    }
}
