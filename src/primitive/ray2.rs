use super::vec2::Vec2;

#[derive(Debug)]
pub struct Ray2 {
    pub pos: Vec2,
    pub dir: Vec2,
}

impl Ray2 {
    pub fn new(pos: Vec2, dir: Vec2) -> Self {
        Ray2 { pos, dir }
    }
}
