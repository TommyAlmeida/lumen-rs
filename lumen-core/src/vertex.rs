use crate::color::Color;
use crate::vector::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct TextureVertex {
    position  : Vector2D,
    tex_coords: Vector2D
}

implement_vertex!(TextureVertex, position, tex_coords);

impl TextureVertex{
    #[inline]
    pub fn new(position: Vector2D, tex_coords: Vector2D) -> Self{
        TextureVertex{
            position,
            tex_coords,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ColorVertex {
    position: Vector2D,
    color: Color,
}

implement_vertex!(ColorVertex, position, color);

impl ColorVertex {
    #[inline]
    pub fn new(position: Vector2D, color: Color) -> Self{
        ColorVertex {
            position,
            color
        }
    }
}