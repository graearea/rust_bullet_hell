use macroquad::prelude::*;

// 1. Data Structures (No logic here, just data)
struct Player {
    pos: Vec2,
    speed: f32,
    size: f32
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
        speed : 200.0,
        size : 20.0
    };
    // let mut frame_count = 0;

    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        // frame_count += 1;
        if is_key_down(KeyCode::Right) { player.pos.x += player.speed * delta_time; }
        if is_key_down(KeyCode::Left)  { player.pos.x -= player.speed * delta_time; }
        if is_key_down(KeyCode::Down)  { player.pos.y += player.speed * delta_time; }
        if is_key_down(KeyCode::Up)    { player.pos.y -= player.speed * delta_time; }

        draw_rectangle(player.pos.x, player.pos.y, player.size, player.size, BLUE);
        next_frame().await
    }
}
