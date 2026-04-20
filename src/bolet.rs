use crate::{Drawable, HasPhysics};
use macroquad::color::{Color, BLACK, BLUE, RED, WHITE, YELLOW};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{draw_circle, draw_text};

pub struct Bolet {
    pub pos: Vec2,
    velocity: Vec2,
    pub hit: bool,
    colour: Color,
}

impl Bolet {
    pub(crate) fn hit(&mut self) {
        self.hit = true;
    }
}

impl HasPhysics for Bolet {
    fn add_velocity(&mut self, singularity: Vec2, delta_time: f32) {
        let direction = singularity - self.pos;
        let distance = direction.length().max(120.0);

        let gravity_strength = 8000000.0;
        let force = gravity_strength / (distance * (distance / 4.0));

        let pull = direction.normalize() * force * delta_time;
        self.velocity += pull;
    }
}

impl Bolet {

    pub fn in_bounds(&self, width: f32, height: f32) -> bool {
        self.pos.x > -1000.0
            && self.pos.x < width + 1000.0
            && self.pos.y > -1000.0
            && self.pos.y < height + 1000.0
    }

    pub fn new(pos: Vec2, speed: f32, direction: f32) -> Self {
        // Convert polar (speed + angle) to cartesian velocity
        Self {
            pos: pos,
            velocity: vec2(direction.cos() * speed, direction.sin() * speed),
            hit: false,
            colour: RED,
        }
    }
}

impl Drawable for Bolet {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 3.0, gradient_color(self.velocity.length()/5.0));
    }

    fn update(&mut self, delta_time: f32) {
        self.pos += self.velocity * delta_time;
    }
}

fn gradient_color(t: f32) -> Color {
    let t = t.clamp(0.0, 100.0) / 100.0;

    let stops: &[(f32, Color)] = &[(0.00, BLUE), (0.33, RED), (0.66, YELLOW), (1.00, WHITE)];

    for i in 0..stops.len() - 1 {
        let (t0, c0) = stops[i];
        let (t1, c1) = stops[i + 1];
        if t <= t1 {
            let s = (t - t0) / (t1 - t0);
            return Color::new(
                c0.r + (c1.r - c0.r) * s,
                c0.g + (c1.g - c0.g) * s,
                c0.b + (c1.b - c0.b) * s,
                1.0,
            );
        }
    }
    WHITE
}
