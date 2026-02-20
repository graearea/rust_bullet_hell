use macroquad::audio::play_sound;
use macroquad::prelude::*;

trait Drawable {
    fn draw(&self);
    fn update(&mut self, delta_time: f32);
}

struct Player {
    pos: Vec2,
    speed: f32,
    size: f32,
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

struct Enema {
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

struct Bolet {
    pos: Vec2,
    speed: f32,
    direction: f32,
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

impl Enema {
    fn shoot_at(&self, target: Vec2) -> Bolet {
        let direction = (target - self.pos).normalize_or_zero();
        Bolet {
            pos: self.pos,
            speed: 200.0,
            direction: direction.y.atan2(direction.x),
        }
    }
}

// Configuration for the window
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Bullet Hell".to_owned(),
        window_width: 640,
        window_height: 480,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut player = Player {
        pos: vec2(200.0, 200.0),
        speed: 200.0,
        size: 20.0,
    };
    let mut enemy = Enema {
        pos: vec2(100.0, 100.0),
        speed: 50.0,
        size: 15.0,
    };

    let mut frame_count = 0;
    let mut things_to_draw: Vec<&mut dyn Drawable> = vec![&mut enemy];
    let mut bullets: Vec<Bolet> = vec![];

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        frame_count += 1;

        // Update and draw player directly
        player.update(delta_time);
        player.draw();

        // Enemy shoots at player
        if rand::gen_range(0, 20) == 0 {
            bullets.push(enemy.shoot_at(player.pos));
        }

        // Update and draw bullets
        for bullet in &mut bullets {
            bullet.update(delta_time);
            bullet.draw();
        }

        enemy.draw();

        next_frame().await;
    }
}
