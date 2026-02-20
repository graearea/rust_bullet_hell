use macroquad::math::{f32, Vec2};
use macroquad::prelude::draw_circle;
use macroquad::color::WHITE;
use crate::Drawable;

pub struct Bolet {
    pub pos: Vec2,
    speed: f32,
    direction: f32,
}

impl Bolet {
    pub fn new(pos: Vec2, speed: f32, direction: f32) -> Self {
        Self { pos, speed, direction }
    }
}

impl Drawable for Bolet {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 3.0, WHITE);
    }

    fn update(&mut self, delta_time: f32) {
        self.pos.x += self.direction.cos() * self.speed * delta_time;
        self.pos.y += self.direction.sin() * self.speed * delta_time;

    }
}