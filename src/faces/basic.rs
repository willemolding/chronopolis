use crate::prelude::*;

#[derive(Debug)]
pub struct BasicFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
    name: &'static str,
    // Resolved once rather than `format!`ed on every frame. Four heap
    // allocations per frame is not what makes this slow, but the target has no
    // cycles going spare either.
    bg_key: String,
    hours_key: String,
    mins_key: String,
    secs_key: String,
}

impl BasicFace {
    pub fn new(name: &'static str, path_prefix: &'static str) -> Self {
        Self {
            hour_hand: ClockHand::new().easing(Easing::Linear).duration(60. * 60.),
            min_hand: ClockHand::new().easing(Easing::Linear).duration(60.),
            sec_hand: ClockHand::sweeping(),
            name,
            bg_key: format!("{path_prefix}/bg"),
            hours_key: format!("{path_prefix}/hours"),
            mins_key: format!("{path_prefix}/mins"),
            secs_key: format!("{path_prefix}/secs"),
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
            ctx.texture(&self.bg_key),
            ctx.texture(&self.hours_key),
            ctx.texture(&self.mins_key),
            ctx.texture(&self.secs_key),
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
