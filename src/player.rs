use nalgebra::Vector3;

use super::color::Color;
use super::input::*;
use super::render_target_3dsprites::RenderTarget3DSprites;

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
        if input.is_key_down(KeyboardKey::KEY_R) {
            self.position[2] += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_F) {
            self.position[2] -= 1.0;
        }
    }

    pub fn draw(&self, target: &mut RenderTarget3DSprites) {
        target.draw_circle(self.position, 64.0, Color::PURPLE);
    }
}
