use crate::bolet::Bolet;
use crate::Drawable;
use macroquad::color::{BLACK, RED};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::draw_circle;
use macroquad::rand::gen_range;

pub struct Hole {
    pos: Vec2,
    velocity: Vec2,
    size: f32,
}

impl Drawable for Hole {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.size, BLACK);
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
    pub fn suck(){}
}
