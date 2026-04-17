use gamestate::GameState;
use macroquad::prelude::*;
use std::process::exit;

mod bolet;
mod enema;
mod gamestate;
mod hole;
mod player;

trait Drawable {
    fn draw(&self);
    fn update(&mut self, delta_time: f32);
}

trait HasPhysics {
    fn add_velocity(&mut self, singularity: Vec2, delta_time: f32);
}

enum Screen {
    Playing,
    GameOver,
}

// Configuration for the window
fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Bullet Hell".to_owned(),
        window_width: 1280,
        window_height: 1024,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::new();
    let mut screen = Screen::Playing;

    loop {
        match screen {
            Screen::Playing => {
                if !game.update() {
                    screen = Screen::GameOver;
                }
                game.draw();
            }
            Screen::GameOver => {
                draw_game_over();
                if is_key_pressed(KeyCode::Escape) {
                    exit(0);
                }
            }
        }
        next_frame().await;
    }

    // Game over screen loop
    fn draw_game_over() {
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

    }
}
