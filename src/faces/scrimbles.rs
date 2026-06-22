use crate::clock_face::ClockFace;
use crate::clock_hand::ClockHand;
use nannou::prelude::*;

#[derive(Debug)]
pub struct ScrimblesFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
}

impl ScrimblesFace {
    pub fn new() -> Self {
        Self {
            hour_hand: ClockHand::smooth(),
            min_hand: ClockHand::smooth(),
            sec_hand: ClockHand::bouncy(),
        }
    }
}

impl ClockFace for ScrimblesFace {
    fn name(&self) -> &str {
        "Scrimbles"
    }

    fn update(&mut self, _app: &App, ctx: &crate::ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, app: &App, ctx: &crate::ClockContext, draw: &Draw) {
        let size = f32::min(app.window_rect().w(), app.window_rect().h());
        let wh = vec2(size, size);

        draw.texture(ctx.texture("scrimbles/bg"))
            .xy(app.window_rect().xy())
            .wh(wh);

        draw.texture(ctx.texture("scrimbles/hours"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.hour_hand.angle());

        draw.texture(ctx.texture("scrimbles/mins"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.min_hand.angle());

        draw.texture(ctx.texture("scrimbles/secs"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.sec_hand.angle());
    }
}
