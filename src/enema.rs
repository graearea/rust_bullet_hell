use crate::bolet::Bolet;
use crate::Drawable;
use macroquad::color::RED;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::draw_circle;
use macroquad::rand::gen_range;

pub struct Enema {
    pos: Vec2,
    velocity: Vec2,
    size: f32,
}

impl Drawable for Enema {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, RED);
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


impl Enema {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            velocity: vec2(0.0, 0.0),
            size: 20.0,
        }
    }
    pub fn shoot_at(&self, target: Vec2) -> Bolet {
        let direction = (target - self.pos).normalize_or_zero();
        Bolet::new(self.pos, 200.0, direction.y.atan2(direction.x))
    }
}
