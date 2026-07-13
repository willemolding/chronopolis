use crate::clock_face::ClockFace;
use crate::clock_hand::ClockHand;
use nannou::prelude::*;

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

    fn update(&mut self, _app: &App, ctx: &crate::ClockContext) {
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
    }

    fn view(&self, app: &App, ctx: &crate::ClockContext, draw: &Draw) {
        let size = f32::min(app.window_rect().w(), app.window_rect().h());
        let wh = vec2(size, size);

        if let (Some(secs), Some(mins), Some(sun)) = (
            ctx.texture("clocktown/secs"),
            ctx.texture("clocktown/mins"),
            ctx.texture("clocktown/sun"),
        ) {
            draw.texture(secs)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.sec_hand.angle());

            draw.texture(mins)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.min_hand.angle());

            draw.texture(sun)
                .xy(app.window_rect().xy())
                .wh(wh)
                .rotate(-self.min_hand.angle());
        } else {
            draw.text("Missing textures")
                .xy(app.window_rect().xy())
                .color(RED)
                .font_size(48);
        }
    }
}
