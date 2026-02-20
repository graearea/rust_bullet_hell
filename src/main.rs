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
    // let mut things_to_draw: Vec<&mut dyn Drawable> = vec![
    //     // &mut Enema::new(vec2(100.0, 400.0), 50.0, 15.0),
    //     // &mut Enema::new(vec2(400.0, 400.0), 50.0, 15.0),
    //     // &mut Enema::new(vec2(100.0, 100.0), 50.0, 15.0)
    // ];
    let mut bullets: Vec<Bolet> = vec![];
    let enemies: Vec<Enema> = vec![
        Enema::new(vec2(100.0, 400.0), 50.0, 15.0),
        Enema::new(vec2(400.0, 400.0), 50.0, 15.0),
        Enema::new(vec2(100.0, 100.0), 50.0, 15.0)
    ];

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        frame_count += 1;

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
            bullet.draw();
        }


        next_frame().await;
    }
}
