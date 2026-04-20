use crate::Drawable;
use macroquad::color::Color;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::draw_circle;
use macroquad::rand::gen_range;

pub struct Hole {
    pub pos: Vec2,
    velocity: Vec2,
    pub(crate) size: f32,
}

impl Drawable for Hole {
    fn draw(&self) {
        // Draw outer rings first (more transparent), inner rings last (more opaque)
        (0..40).rev().for_each(|step| {
            let t = (step as f32) / 40.0;  // 0.0 at outer edge, 1.0 at center
            let alpha = t * t * t * 0.8;   // cubic falloff - very subtle at edges, ramps up fast
            draw_circle(
                self.pos.x,
                self.pos.y,
                self.size*2.0 * (4.0 - t * 3.5), // size grows outward
                Color::new(0.0, 0.0, 0.0, alpha),
            );
        });
        // Solid core
        draw_circle(
            self.pos.x,
            self.pos.y,
            self.size * 0.5,
            Color::new(0.0, 0.0, 0.0, 1.0),
        );
    }

    fn update(&mut self, delta_time: f32) {
        self.velocity.x += gen_range(-50.0, 50.0) * delta_time;
        self.velocity.y += gen_range(-50.0, 50.0) * delta_time;
        let max_speed = 100.0;
        self.velocity = self.velocity.clamp_length_max(max_speed);
        // println!("enema speed{}" , self.speed);
        self.pos += self.velocity * delta_time;
        // println!("enema pos{}" , self.pos);
    }
}

impl Hole {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            velocity: vec2(0.0, 0.0),
            size: 20.0,
        }
    }
    pub fn suck() {}
}
