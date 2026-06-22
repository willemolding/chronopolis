use crate::clock_face::ClockFace;
use crate::clock_hand::ClockHand;
use nannou::prelude::*;

#[derive(Debug)]
pub struct AnalogFace {
    hour_hand: ClockHand,
    min_hand: ClockHand,
    sec_hand: ClockHand,
}

impl AnalogFace {
    pub fn new() -> Self {
        Self {
            hour_hand: ClockHand::smooth(),
            min_hand: ClockHand::smooth(),
            sec_hand: ClockHand::bouncy(),
        }
    }
}

impl ClockFace for AnalogFace {
    fn name(&self) -> &str {
        "Analog"
    }

    fn update(&mut self, _app: &App, ctx: &crate::ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, _app: &App, ctx: &crate::ClockContext, draw: &Draw) {
        // Clock face
        draw.ellipse()
            .x_y(0.0, 0.0)
            .radius(ctx.radius)
            .color(rgb(0.15, 0.15, 0.2))
            .stroke(WHITE)
            .stroke_weight(2.0);

        // Hour tick marks
        for i in 0..12 {
            let angle = i as f32 / 12.0 * TAU;
            let (sin, cos) = angle.sin_cos();
            let inner = ctx.radius * 0.88;
            let outer = ctx.radius * 0.97;
            draw.line()
                .start(pt2(sin * inner, cos * inner))
                .end(pt2(sin * outer, cos * outer))
                .weight(2.5)
                .color(WHITE);
        }

        // Minute tick marks
        for i in 0..60 {
            if i % 5 == 0 {
                continue;
            } // skip hour positions
            let angle = i as f32 / 60.0 * TAU;
            let (sin, cos) = angle.sin_cos();
            let inner = ctx.radius * 0.93;
            let outer = ctx.radius * 0.97;
            draw.line()
                .start(pt2(sin * inner, cos * inner))
                .end(pt2(sin * outer, cos * outer))
                .weight(1.0)
                .color(GRAY);
        }

        // Hour hand
        let (s, c) = self.hour_hand.angle().sin_cos();
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(s * ctx.radius * 0.55, c * ctx.radius * 0.55))
            .weight(6.0)
            .caps_round()
            .color(WHITE);

        // Minute hand
        let (s, c) = self.min_hand.angle().sin_cos();
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(s * ctx.radius * 0.78, c * ctx.radius * 0.78))
            .weight(4.0)
            .caps_round()
            .color(WHITE);

        // Second hand
        let (s, c) = self.sec_hand.angle().sin_cos();
        draw.line()
            .start(pt2(-s * ctx.radius * 0.15, -c * ctx.radius * 0.15)) // tail
            .end(pt2(s * ctx.radius * 0.88, c * ctx.radius * 0.88))
            .weight(1.5)
            .color(RED);

        // Centre dot
        draw.ellipse().x_y(0.0, 0.0).radius(4.0).color(RED);
    }
}
