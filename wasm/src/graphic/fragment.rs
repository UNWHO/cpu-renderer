use super::color::Color;
use crate::math::vector::Vector2;

pub struct Fragment {
    pub pos: Vector2<usize>,
    pub depth: f64,
    pub color: Color,
}
