use crate::primitive::{Ray2, Vec2};

#[derive(Debug)]
pub struct Camera {
    pub pos: Vec2,
    pub dir: Vec2,
    pub fov: f32,
    plane: Vec2,
}

impl Camera {
    pub fn new(pos: Vec2, dir: Vec2, fov: f32) -> Self {
        Camera {
            pos,
            fov,
            dir,
            plane: Vec2::new(-dir.y, dir.x) * (fov / 2.0).to_radians().tan(),
        }
    }

    pub fn ray(&self, pct_x: f32) -> Ray2 {
        Ray2::new(self.pos, (self.dir + pct_x * self.plane).normalized())
    }

    pub fn rotate_by(&mut self, rad: f32) {
        self.dir = self.dir.rotated(rad);
        self.plane = Vec2::new(-self.dir.y, self.dir.x) * (self.fov / 2.0).to_radians().tan();
    }
}
