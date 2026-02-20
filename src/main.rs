use macroquad::prelude::*;

// 1. Data Structures (No logic here, just data)
struct Player {
    pos: Vec2,
    speed: f32,
    size: f32,
}

struct Bullet {
    pos: Vec2,
    vel: Vec2,
    alive: bool,
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
    // Initialize our "Objects"
    let mut player = Player {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        speed: 300.0,
        size: 20.0,
    };

    let mut bullets: Vec<Bullet> = Vec::new();
    let mut frame_count = 0;

    // THE GAME LOOP
    loop {
        clear_background(BLACK);
        let delta_time = get_frame_time(); // Equivalent to 'dt' in other engines
        frame_count += 1;

        // --- INPUT & MOVEMENT ---
        if is_key_down(KeyCode::Right) { player.pos.x += player.speed * delta_time; }
        if is_key_down(KeyCode::Left)  { player.pos.x -= player.speed * delta_time; }
        if is_key_down(KeyCode::Down)  { player.pos.y += player.speed * delta_time; }
        if is_key_down(KeyCode::Up)    { player.pos.y -= player.speed * delta_time; }

        // --- SPAWN BULLETS (The "Hell" part) ---
        // Every 5 frames, spawn a bullet moving downwards
        if frame_count % 5 == 0 {
            bullets.push(Bullet {
                pos: vec2(rand::gen_range(0.0, screen_width()), 0.0),
                vel: vec2(0.0, 400.0),
                alive: true,
            });
        }

        // --- UPDATE BULLETS ---
        // In Java, you'd do bullet.update(). In Rust, we iterate and mutate.
        for bullet in bullets.iter_mut() {
            bullet.pos += bullet.vel * delta_time;

            // Simple Collision Detection (Circle vs Rectangle-ish)
            if f32::abs(bullet.pos.x - player.pos.x) < player.size &&
               f32::abs(bullet.pos.y - player.pos.y) < player.size {
                // In a real game, you'd trigger a Game Over here
                bullet.alive = false;
            }

            // Screen boundary check
            if bullet.pos.y > screen_height() {
                bullet.alive = false;
            }
        }

        // --- CLEANUP ---
        // This is the "Rust Way" to handle lists. Efficiently remove "dead" bullets.
        bullets.retain(|b| b.alive);

        // --- DRAWING ---
        draw_rectangle(player.pos.x, player.pos.y, player.size, player.size, BLUE);

        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 4.0, RED);
        }

        // Wait for next frame
        next_frame().await
    }
}
