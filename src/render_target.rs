use nalgebra::Vector2;
use raylib::{drawing::RaylibDrawHandle, prelude::RaylibDraw};

use super::color::*;

pub struct RenderTarget<'a, 'b: 'a> {
    handle: &'a mut RaylibDrawHandle<'b>,
}

impl<'a, 'b: 'a> RenderTarget<'a, 'b> {
    pub fn new(handle: &'a mut RaylibDrawHandle<'b>) -> Self {
        RenderTarget { handle: handle }
    }

    pub fn clear_background(&mut self, color: Color) {
        self.handle.clear_background(color);
    }

    pub fn draw_circle(&mut self, position: Vector2<f32>, radius: f32, color: Color) {
        self.handle.draw_circle(
            position[0].round() as i32,
            position[1].round() as i32,
            radius,
            color,
        );
    }
}
