use nalgebra::Vector3;

use super::color::Color;
use super::input::*;
use super::render_target::RenderTarget;

pub struct Player {
    position: Vector3<f32>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, input: &Input) {
        if input.is_key_down(KeyboardKey::KEY_W) {
            self.position[1] += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_S) {
            self.position[1] -= 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_A) {
            self.position[0] -= 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_D) {
            self.position[0] += 1.0;
        }
    }

    pub fn draw(&self, target: &mut RenderTarget) {
        let mut position_xy = self.position.fixed_rows::<2>(0).into_owned();
        position_xy[1] = -position_xy[1];
        target.draw_circle(position_xy, 128.0, Color::RED);
    }
}
