use super::color::Color;
use super::render_target_3dsprites::RenderTarget3DSprites;
use nalgebra::{Vector2, Vector3};

pub struct TargetDummy {
    position: Vector3<f32>,
}

impl TargetDummy {
    pub fn new() -> TargetDummy {
        TargetDummy {
            position: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn draw(&self, target: &mut RenderTarget3DSprites) {
        target.draw_rectangle(self.position, Vector2::new(40.0, 80.0), Color::BROWN)
    }
}
