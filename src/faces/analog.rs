use crate::prelude::*;

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

    fn update(&mut self, ctx: &ClockContext) {
        self.hour_hand.animate_to(ctx.hour_angle, ctx.dt);
        self.min_hand.animate_to(ctx.min_angle, ctx.dt);
        self.sec_hand.animate_to(ctx.sec_angle, ctx.dt);
    }

    fn view(&self, ctx: &ClockContext) {
        // `draw_circle` is only a 20-gon, which is visibly faceted at this
        // size, so go round the long way.
        const SIDES: u8 = 255;

        // Clock face
        draw_poly(
            0.0,
            0.0,
            SIDES,
            ctx.radius,
            0.0,
            Color::new(0.15, 0.15, 0.2, 1.0),
        );
        draw_poly_lines(0.0, 0.0, SIDES, ctx.radius, 0.0, 2.0, WHITE);

        // Hour tick marks
        for i in 0..12 {
            let angle = i as f32 / 12.0 * TAU;
            let inner = polar(angle, ctx.radius * 0.88);
            let outer = polar(angle, ctx.radius * 0.97);
            draw_line(inner.x, inner.y, outer.x, outer.y, 2.5, WHITE);
        }

        // Minute tick marks
        for i in 0..60 {
            if i % 5 == 0 {
                continue;
            } // skip hour positions
            let angle = i as f32 / 60.0 * TAU;
            let inner = polar(angle, ctx.radius * 0.93);
            let outer = polar(angle, ctx.radius * 0.97);
            draw_line(inner.x, inner.y, outer.x, outer.y, 1.0, GRAY);
        }

        // Hour hand
        draw_line_round(
            Vec2::ZERO,
            polar(self.hour_hand.angle(), ctx.radius * 0.55),
            6.0,
            WHITE,
        );

        // Minute hand
        draw_line_round(
            Vec2::ZERO,
            polar(self.min_hand.angle(), ctx.radius * 0.78),
            4.0,
            WHITE,
        );

        // Second hand, with a tail poking out the back
        let sec = self.sec_hand.angle();
        let tail = polar(sec, ctx.radius * -0.15);
        let tip = polar(sec, ctx.radius * 0.88);
        draw_line(tail.x, tail.y, tip.x, tip.y, 1.5, RED);

        // Centre dot
        draw_circle(0.0, 0.0, 4.0, RED);
    }
}
