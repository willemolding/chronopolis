use crate::clock_context::ClockContext;
use crate::clock_face::ClockFace;
use crate::clock_hand::ClockHand;
use nannou::prelude::*;

#[derive(Debug)]
pub struct PurpcrystFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
}

impl PurpcrystFace {
    pub fn new() -> Self {
        Self {
            hour_hand: ClockHand::sweeping(),
            min_hand: ClockHand::sweeping(),
            sec_hand: ClockHand::springy(),
        }
    }
}

impl ClockFace for PurpcrystFace {
    fn name(&self) -> &str {
        "Purpcryst"
    }

    fn update(&mut self, _app: &App, ctx: &ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, app: &App, ctx: &ClockContext, draw: &Draw) {
        let size = f32::min(app.window_rect().w(), app.window_rect().h());
        let wh = vec2(size, size);

        draw.texture(ctx.texture("purpcryst/bg"))
            .xy(app.window_rect().xy())
            .wh(wh);

        draw.texture(ctx.texture("purpcryst/secs"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.sec_hand.angle());

        draw.texture(ctx.texture("purpcryst/mins"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.min_hand.angle());

        draw.texture(ctx.texture("purpcryst/hours"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.hour_hand.angle());
    }
}
