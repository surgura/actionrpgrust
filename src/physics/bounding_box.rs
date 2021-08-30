// 3d rectangle

#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub x: f32, // dist from center to one side on x axis
    pub y: f32,
    pub z: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, z: f32) -> BoundingBox {
        BoundingBox { x, y, z }
    }
}
