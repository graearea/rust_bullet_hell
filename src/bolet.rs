use macroquad::math::{vec2, Vec2};
use macroquad::prelude::draw_circle;
use macroquad::color::WHITE;
use crate::{Drawable, HasPhysics};

pub struct Bolet {
    pub pos: Vec2,
    velocity: Vec2,
}

impl HasPhysics for Bolet {
    fn add_velocity(&mut self, singularity: Vec2, delta_time: f32) {
        let direction = singularity - self.pos;
        let distance = direction.length().max(60.0);

        let gravity_strength = 2000000.0;
        let force = gravity_strength / (distance * distance);

        let pull = direction.normalize() * force * delta_time;
        self.velocity += pull;
    }
}

impl Bolet {
    pub fn new(pos: Vec2, speed: f32, direction: f32) -> Self {
        // Convert polar (speed + angle) to cartesian velocity
        Self {
            pos,
            velocity: vec2(direction.cos() * speed, direction.sin() * speed)
        }
    }
}

impl Drawable for Bolet {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 3.0, WHITE);
    }

    fn update(&mut self, delta_time: f32) {
        self.pos += self.velocity * delta_time;
    }
}
