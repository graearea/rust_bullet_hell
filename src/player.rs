use macroquad::prelude::{draw_rectangle, draw_text};
use macroquad::color::{BLACK, BLUE, RED};
use macroquad::math::{f32, Vec2};
use macroquad::input::{is_key_down, KeyCode};
use crate::Drawable;

pub struct Player {
    pub(crate) pos: Vec2,
    velocity: f32,
    pub(crate) size: f32,
    pub(crate) health: i32,
}

impl Player {
    pub(crate) fn damage(&mut self) {
        self.health=self.health-1
    }
}

impl Player{
    pub fn new(pos: Vec2,
           speed: f32,
           size: f32,
           health: i32) -> Self {
        Self { pos, velocity: speed, size, health }
    }
}

impl Drawable for Player {
    fn draw(&self) {
        draw_rectangle(self.pos.x-(self.size / 2.0), self.pos.y-(self.size / 2.0), self.size, self.size, BLUE);
        draw_text(
            &format!("{:.0}", self.health),
            self.pos.x-(self.size / 2.0),
            self.pos.y+(self.size / 2.0),
            20.0,
            BLACK,
        );
    }
    fn update(&mut self, delta_time: f32) {
        if is_key_down(KeyCode::Right) {
            self.pos.x += self.velocity * delta_time;
        }
        if is_key_down(KeyCode::Left) {
            self.pos.x -= self.velocity * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            self.pos.y += self.velocity * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            self.pos.y -= self.velocity * delta_time;
        }
    }
}