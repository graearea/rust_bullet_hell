use std::process::exit;
use bolet::Bolet;
use enema::Enema;
use macroquad::prelude::*;
use player::Player;

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

    let mut frame_count = 0;

    let mut bullets: Vec<Bolet> = vec![];
    let enemies: Vec<Enema> = vec![
        Enema::new(vec2(100.0, 400.0), 50.0, 15.0),
        Enema::new(vec2(400.0, 400.0), 50.0, 15.0),
        Enema::new(vec2(100.0, 100.0), 50.0, 15.0)
    ];
    let start_time = get_time();
    let mut new_time = get_time();
    let mut fps = 1.0;
    'game: loop {
        clear_background(BLACK);
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        let elapsed_time = get_time() - start_time;
        frame_count += 1;


        if (get_time() > new_time){
            println!("here");
            fps = 1.0 / delta_time;
            new_time = get_time()+1.0
        }
        draw_text(
            &format!("FPS: {:.0}  Time: {:.1}s", fps, elapsed_time),
            10.0, 10.0,
            20.0,
            RED,
        );

        // Update and draw player directly
        player.update(delta_time);
        player.draw();

        // Enemy shoots at player
        for enema in &enemies {
            if rand::gen_range(0, 20) == 0 {
                bullets.push(enema.shoot_at(player.pos));
            }
            enema.draw()
        }

        // Update and draw bullets
        for bullet in &mut bullets {
            bullet.update(delta_time);
            let distance = bullet.pos.distance(player.pos);
            println!("Distance: {}", distance);
            if distance < 5.0 {
                break 'game
            }
            bullet.draw();
        }


        next_frame().await;
    }

    // Game over screen loop
    loop {
        clear_background(BLACK);

        let text = "GAME OVER";
        let font_size = 60.0;
        let text_size = measure_text(text, None, font_size as u16, 1.0);
        draw_text(
            text,
            screen_width() / 2.0 - text_size.width / 2.0,
            screen_height() / 2.0,
            font_size,
            RED,
        );

        let restart_text = "press ESC to Quit";
        let restart_size = measure_text(restart_text, None, 20, 1.0);
        draw_text(
            restart_text,
            screen_width() / 2.0 - restart_size.width / 2.0,
            screen_height() / 2.0 + 50.0,
            20.0,
            WHITE,
        );

        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }

        next_frame().await;
    }
}
