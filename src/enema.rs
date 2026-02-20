use crate::bolet::Bolet;
use crate::Drawable;
use macroquad::color::RED;
use macroquad::math::{f32, Vec2};
use macroquad::prelude::draw_circle;

pub struct Enema {
    pos: Vec2,
    speed: f32,
    size: f32,
}

impl Drawable for Enema {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, RED);
    }

    fn update(&mut self, delta_time: f32) {
        todo!()
    }
}

impl Enema {
    pub fn new(pos: Vec2, speed: f32, size: f32) -> Self {
        Self { pos, speed, size }
    }
    pub fn shoot_at(&self, target: Vec2) -> Bolet {
        let direction = (target - self.pos).normalize_or_zero();
        Bolet::new(self.pos, 200.0, direction.y.atan2(direction.x))
    }
}
