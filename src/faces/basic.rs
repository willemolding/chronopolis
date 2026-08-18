use crate::prelude::*;

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

    fn update(&mut self, ctx: &ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, ctx: &ClockContext) {
        let size = ctx.canvas();

        if let (Some(bg), Some(hours), Some(mins), Some(secs)) = (
            ctx.texture(&format!("{}/bg", self.path_prefix)),
            ctx.texture(&format!("{}/hours", self.path_prefix)),
            ctx.texture(&format!("{}/mins", self.path_prefix)),
            ctx.texture(&format!("{}/secs", self.path_prefix)),
        ) {
            draw_texture_centred(bg, Vec2::ZERO, size, 0.0);
            draw_texture_centred(hours, Vec2::ZERO, size, self.hour_hand.angle());
            draw_texture_centred(mins, Vec2::ZERO, size, self.min_hand.angle());
            draw_texture_centred(secs, Vec2::ZERO, size, self.sec_hand.angle());
        } else {
            draw_text_centred("Missing textures", Vec2::ZERO, 48, RED);
        }
    }
}
