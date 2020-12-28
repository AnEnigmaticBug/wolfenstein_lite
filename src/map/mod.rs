use crate::primitive::{Ray2, Vec2};

pub type TexId = u8;

pub struct Map {
    pub wd: usize,
    pub ht: usize,
    pub grid: Vec<TexId>,
}

#[derive(Debug)]
pub struct Intersection {
    pub pos: Vec2,
    pub tex: TexId,
    pub in_ns_dir: bool,
}

impl Map {
    pub fn intersect(&self, ray: &Ray2) -> Intersection {
        let tan = ray.dir.y / ray.dir.x;
        let cot = 1.0 / tan;
        let dir = Vec2::new(ray.dir.x.signum(), ray.dir.y.signum());

        let step_ew = Vec2::new(cot.abs() * dir.x, dir.y);
        let step_ns = Vec2::new(dir.x, tan.abs() * dir.y);

        // Potential intersection position with a EW wall.
        let mut pos_ew = if dir.y > 0.0 {
            let shift = ray.pos.y.ceil() - ray.pos.y;
            Vec2::new(ray.pos.x + shift * cot, ray.pos.y.ceil())
        } else {
            let shift = -ray.pos.y.fract();
            Vec2::new(ray.pos.x + shift * cot, ray.pos.y.floor())
        };

        // Potential intersection position with a NS wall.
        let mut pos_ns = if dir.x > 0.0 {
            let shift = ray.pos.x.ceil() - ray.pos.x;
            Vec2::new(ray.pos.x.ceil(), ray.pos.y + shift * tan)
        } else {
            let shift = -ray.pos.x.fract();
            Vec2::new(ray.pos.x.floor(), ray.pos.y + shift * tan)
        };

        loop {
            if (pos_ew.x * dir.x) < (pos_ns.x as f32 * dir.x) {
                let res = pos_ew;
                pos_ew.x += step_ew.x;
                pos_ew.y += step_ew.y;

                let idx_x = res.x as usize;
                let idx_y = if dir.y > 0.0 { res.y } else { res.y - 1.0 } as usize;
                let tex = self.grid[self.wd * idx_y + idx_x];

                if tex != 0 {
                    return Intersection {
                        pos: res,
                        tex,
                        in_ns_dir: false,
                    };
                }
            } else {
                let res = pos_ns;
                pos_ns.x += step_ns.x;
                pos_ns.y += step_ns.y;

                let idx_x = if dir.x > 0.0 { res.x } else { res.x - 1.0 } as usize;
                let idx_y = res.y as usize;
                let tex = self.grid[self.wd * idx_y + idx_x];

                if tex != 0 {
                    return Intersection {
                        pos: res,
                        tex,
                        in_ns_dir: true,
                    };
                }
            }
        }
    }
}
