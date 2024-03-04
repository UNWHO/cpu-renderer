use super::color::Color;
use crate::math::vector::Vector3;

pub struct Vertex {
    pub pos: Vector3<f64>,
    pub color: Color,
}

impl Vertex {
    pub fn new(pos: Vector3<f64>, color: Color) -> Self {
        Self { pos, color }
    }

    pub fn from_array(pos: &[f64; 3], color: &[f64; 4]) -> Self {
        Self::new(
            Vector3::new(pos[0], pos[1], pos[2]),
            Color::new(color[0], color[1], color[2], color[3]),
        )
    }
}
