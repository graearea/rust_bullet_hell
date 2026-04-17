use crate::HasPhysics;
use macroquad::color::{DARKPURPLE, RED};
use macroquad::math::{f32, f64, u32, vec2};
use macroquad::miniquad::date::now;
use macroquad::prelude::{clear_background, draw_text, get_frame_time, get_time, next_frame, rand, screen_height, screen_width};
use crate::bolet::Bolet;
use crate::enema::Enema;
use crate::hole::Hole;
use crate::player::Player;
use crate::Drawable;  // Add this to gamestate.rs

pub struct GameState {
    player: Player,
    bullets: Vec<Bolet>,
    enemies: Vec<Enema>,
    holes: Vec<Hole>,
    frame_count: u32,
    start_time: f64,
    fps_update_time: f64,
    fps: f32,
}

impl GameState {
    pub(crate) fn draw(&self) {
        clear_background(DARKPURPLE);
        draw_text(
            &format!("FPS: {:.0}  Time: {:.1}s", self.fps, now()-self.start_time),
            10.0,
            10.0,
            20.0,
            RED,
        );
        self.player.draw();
        self.enemies.iter().for_each(|e| e.draw());
        self.holes.iter().for_each(|h| h.draw());
        self.bullets.iter().for_each(|b| b.draw());
    }
}

impl GameState {
    pub(crate) fn update(&mut self) -> bool {
        let mut new_time = get_time();
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        let elapsed_time = get_time() - self.start_time;
        self.frame_count += 1;
        if (get_time() > new_time) {
            println!("here");
            self.fps = 1.0 / delta_time;
            new_time = get_time() + 1.0
        }
        self.player.update(delta_time);

        for enema in &mut self.enemies {
            enema.update(delta_time);
            if rand::gen_range(0, 20) == 0 {
                self.bullets.push(enema.shoot_at(self.player.pos));
            }
        }

        for bullet in &mut self.bullets {
            for hole in &self.holes{
                bullet.add_velocity(hole.pos, delta_time)
            }
            bullet.update(delta_time);
            let distance = bullet.pos.distance(self.player.pos);
            // println!("Distance: {}", distance);
            if distance < self.player.size / 2.0 {
                self.player.damage();
                bullet.hit()
            }
        }

        self.bullets.retain(|bullet| {
            let mut sucked=false;
            for hole in &self.holes{
                if bullet.pos.distance(hole.pos) < hole.size {
                    sucked= true;
                }
            }
            !(!bullet.in_bounds(screen_width(),screen_height())
                || bullet.hit
                || sucked
            )

        });

        self.player.health>0

    }
}

impl GameState {
    pub(crate) fn new() -> Self {
        let mut player = Player::new(vec2(200.0, 200.0), 200.0, 20.0,100);

        let mut bullets: Vec<Bolet> = vec![];
        let mut enemies: Vec<Enema> = vec![
            Enema::new(vec2(100.0, 400.0)),
            Enema::new(vec2(400.0, 400.0)),
            Enema::new(vec2(100.0, 100.0)),
        ];

        let holes: Vec<Hole> = vec![Hole::new(vec2(300.0,200.0))];

        let start_time = get_time();

        Self{
            player,
            bullets,
            enemies,
            holes,
            frame_count: 0,
            start_time,
            fps_update_time: 0.0,
            fps: 1.0,
        }
    }
}
