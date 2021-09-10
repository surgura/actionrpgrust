use nalgebra::Vector3;

use super::color::Color;
use super::input::*;
use super::physics;
use super::render_target_3dsprites::RenderTarget3DSprites;

pub struct Player {
    phys_obj: physics::Object,
    left_foot_animation_state: u32,
    anim_timer: u32,
    left_foot_pos: Vector3<f32>,
}

impl Player {
    pub fn new(phys: &mut physics::Environment) -> Player {
        Player {
            phys_obj: phys.new_object(physics::BoundingBox::new(16.0, 16.0, 1.0)),
            left_foot_animation_state: 0,
            anim_timer: 0,
            left_foot_pos: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn update(&mut self, input: &Input) {
        let mut new_velocity = Vector3::new(0.0, 0.0, 0.0);
        if input.is_key_down(KeyboardKey::KEY_UP) {
            new_velocity[1] += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_DOWN) {
            new_velocity[1] -= 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_LEFT) {
            new_velocity[0] -= 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_RIGHT) {
            new_velocity[0] += 1.0;
        }
        if input.is_key_down(KeyboardKey::KEY_R) {
            self.phys_obj
                .set_position(self.phys_obj.get_position() + Vector3::new(0.0, 0.0, 1.0));
        }
        if input.is_key_down(KeyboardKey::KEY_F) {
            self.phys_obj
                .set_position(self.phys_obj.get_position() + Vector3::new(0.0, 0.0, -1.0));
        }
        self.phys_obj.set_velocity(new_velocity);

        self.anim_timer += 1;
        if self.anim_timer >= 8 {
            self.anim_timer = 0;
            self.left_foot_animation_state += 1;
            self.left_foot_animation_state %= 3;
        }

        let anim_up = [
            Vector3::new(-6.0, 0.0, 8.0),
            Vector3::new(-6.0, 8.0, 0.0),
            Vector3::new(-6.0, -8.0, 0.0),
        ];

        let anim_right = [
            Vector3::new(0.0, -6.0, 8.0),
            Vector3::new(8.0, -6.0, 0.0),
            Vector3::new(-8.0, -6.0, 0.0),
        ];

        let anim = anim_right;

        let target: &Vector3<f32> = &anim[self.left_foot_animation_state as usize];

        self.left_foot_pos += (target - self.left_foot_pos) * 0.2;
    }

    pub fn draw(&self, target: &mut RenderTarget3DSprites) {
        /*target.draw_circle(
            self.phys_obj.get_position() + Vector3::new(0.0, 0.0, 12.0),
            16.0,
            Color::PURPLE,
        );
        target.draw_circle(
            self.phys_obj.get_position() + self.left_foot_pos,
            8.0,
            Color::GREEN,
        );*/

        target.draw_circle(self.phys_obj.get_position(), 16.0, Color::GREEN);
    }
}
