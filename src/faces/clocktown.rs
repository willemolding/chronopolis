use crate::prelude::*;

#[derive(Debug)]
pub struct ClocktownFace {
    sec_hand: ClockHand,
    min_hand: ClockHand,
}

impl ClocktownFace {
    pub fn new() -> Self {
        Self {
            sec_hand: ClockHand::bouncy(),
            min_hand: ClockHand::smooth(),
        }
    }
}

impl ClockFace for ClocktownFace {
    fn name(&self) -> &str {
        "Clocktown"
    }

    fn update(&mut self, ctx: &ClockContext) {
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
    }

    fn view(&self, ctx: &ClockContext) {
        let size = ctx.canvas();

        if let (Some(secs), Some(mins), Some(sun)) = (
            ctx.texture("clocktown/secs"),
            ctx.texture("clocktown/mins"),
            ctx.texture("clocktown/sun"),
        ) {
            draw_texture_centred(secs, Vec2::ZERO, size, self.sec_hand.angle());
            draw_texture_centred(mins, Vec2::ZERO, size, self.min_hand.angle());
            draw_texture_centred(sun, Vec2::ZERO, size, self.min_hand.angle());
        } else {
            draw_text_centred("Missing textures", Vec2::ZERO, 48, RED);
        }
    }
}
