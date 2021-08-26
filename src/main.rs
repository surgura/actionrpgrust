use nalgebra::Vector3;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    rl.set_target_fps(60);

    let mut position = Vector3::new(1.0f32, 1.0f32, 1.0f32);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_W) {
            position[1] += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            position[1] -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            position[0] -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            position[0] += 1.0;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_circle(
            position[0].round() as i32,
            -position[1].round() as i32,
            128.0,
            Color::RED,
        );
    }
}
