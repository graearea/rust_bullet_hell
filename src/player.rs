use macroquad::prelude::draw_rectangle;
use macroquad::color::BLUE;
use macroquad::math::{f32, Vec2};
use macroquad::input::{is_key_down, KeyCode};
use crate::Drawable;

pub struct Player {
    pub pos: Vec2,
    speed: f32,
    size: f32,
}
impl Player{
    pub fn new(pos: Vec2,
           speed: f32,
           size: f32) -> Self {
        Self { pos, speed, size }
    }

}
impl Drawable for Player {
    fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, self.size, self.size, BLUE);
    }
    fn update(&mut self, delta_time: f32) {
        if is_key_down(KeyCode::Right) {
            self.pos.x += self.speed * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            self.pos.x -= self.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            self.pos.y += self.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            self.pos.y -= self.speed * delta_time;
        }
    }
}