use super::color::Color;
use super::render_target::RenderTarget;
use nalgebra::{Vector2, Vector3};

pub struct RenderTarget3DSprites<'a, 'b: 'a> {
    render_target: &'a mut RenderTarget<'a, 'b>,
    camera_pos: Vector2<f32>,
    drawn_circles: Vec<DrawnCircle>,
}

struct DrawnCircle {
    position: Vector3<f32>,
    radius: f32,
    color: Color,
}

impl<'a, 'b: 'a> RenderTarget3DSprites<'a, 'b> {
    pub fn new(render_target: &'a mut RenderTarget<'a, 'b>, camera_pos: Vector2<f32>) -> Self {
        RenderTarget3DSprites {
            render_target,
            camera_pos,
            drawn_circles: Vec::new(),
        }
    }

    pub fn draw_circle(&mut self, position: Vector3<f32>, radius: f32, color: Color) {
        self.drawn_circles.push(DrawnCircle {
            position,
            radius,
            color,
        })
    }

    fn execute_prepared_draw_statements(&mut self) {
        let calc_z = |pos: Vector3<f32>| -pos[1] + pos[2];
        self.drawn_circles
            .sort_unstable_by(|a, b| calc_z(a.position).partial_cmp(&calc_z(b.position)).unwrap());
        for circle in &self.drawn_circles {
            let mut position_xy = circle.position.fixed_rows::<2>(0).into_owned();
            position_xy[1] = -position_xy[1];
            position_xy[1] -= circle.position[2];
            position_xy += self.camera_pos;
            self.render_target
                .draw_circle(position_xy, circle.radius, circle.color);
        }
    }
}

impl<'a, 'b: 'a> Drop for RenderTarget3DSprites<'a, 'b> {
    fn drop(&mut self) {
        self.execute_prepared_draw_statements();
    }
}
