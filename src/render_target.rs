use nalgebra::Vector2;
use raylib::prelude::{RaylibDraw, RaylibDrawHandle, RaylibShaderMode, RaylibShaderModeExt};

use super::color::*;
use super::shader::Shader;

pub struct RenderTarget<'a, 'b> {
    handle: &'a mut RaylibDrawHandle<'b>,
}

impl<'a, 'b> RenderTarget<'a, 'b> {
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

    #[must_use]
    pub fn begin_shader_mode<'s>(&'s mut self, shader: &'s Shader) -> ShaderMode<'s, 'b> {
        ShaderMode {
            raylib_shader_mode: self.handle.begin_shader_mode(&shader.raylib_shader),
        }
    }
}

pub struct ShaderMode<'a, 'b> {
    raylib_shader_mode: RaylibShaderMode<'a, RaylibDrawHandle<'b>>,
}

impl<'a, 'b> ShaderMode<'a, 'b> {
    pub fn draw_circle(&mut self, position: Vector2<f32>, radius: f32, color: Color) {
        self.raylib_shader_mode.draw_circle(
            position[0].round() as i32,
            position[1].round() as i32,
            radius,
            color,
        );
    }
}
