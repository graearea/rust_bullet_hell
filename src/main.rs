use macroquad::prelude::*;
use player::Player;
use enema::Enema;
use bolet::Bolet;

mod player;
mod enema;
mod bolet;

trait Drawable {
    fn draw(&self);
    fn update(&mut self, delta_time: f32);
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
    let mut player = Player::new(vec2(200.0, 200.0),
         200.0,
         20.0,
    );
    let mut enemy = Enema::new(
         vec2(100.0, 100.0),
         50.0,
         15.0,
    );

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
