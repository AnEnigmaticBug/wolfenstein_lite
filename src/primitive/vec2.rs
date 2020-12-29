use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn normalized(&self) -> Self {
        self / self.len()
    }
}

impl_op_ex!(- |a: &Vec2| -> Vec2 { -1.0 * a });

impl_op_ex_commutative!(* |a: &Vec2, b: f32| -> Vec2 { Vec2::new(a.x * b, a.y * b) });
impl_op_ex_commutative!(/ |a: &Vec2, b: f32| -> Vec2 { Vec2::new(a.x / b, a.y / b) });

impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x + b.x, a.y + b.y) });
impl_op_ex!(- |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x - b.x, a.y - b.y) });
impl_op_ex!(* |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x * b.x, a.y * b.y) });
impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2::new(a.x / b.x, a.y / b.y) });
