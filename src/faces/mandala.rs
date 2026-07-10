use crate::clock_face::ClockFace;
use crate::clock_hand::{ClockHand, Easing};
use nannou::prelude::*;

#[derive(Debug)]
pub struct MandalaFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
}

impl MandalaFace {
    pub fn new() -> Self {
        Self {
            hour_hand: ClockHand::new().easing(Easing::Linear).duration(60. * 60.),
            min_hand: ClockHand::new().easing(Easing::Linear).duration(60.),
            sec_hand: ClockHand::sweeping(),
        }
    }
}

impl ClockFace for MandalaFace {
    fn name(&self) -> &str {
        "Mandala"
    }

    fn update(&mut self, _app: &App, ctx: &crate::ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, app: &App, ctx: &crate::ClockContext, draw: &Draw) {
        let size = f32::min(app.window_rect().w(), app.window_rect().h());
        let wh = vec2(size, size);

        draw.texture(ctx.texture("mandala/bg"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.sec_hand.angle());

        draw.texture(ctx.texture("mandala/mins"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.min_hand.angle());

        draw.texture(ctx.texture("mandala/hours"))
            .xy(app.window_rect().xy())
            .wh(wh)
            .rotate(-self.hour_hand.angle());
    }
}
