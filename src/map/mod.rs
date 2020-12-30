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
    pub fn resolve_collisions(&self, old_pos: Vec2, new_pos: Vec2) -> Vec2 {
        let old_idx_x = old_pos.x as usize;
        let old_idx_y = old_pos.y as usize;
        let new_idx_x = new_pos.x as usize;
        let new_idx_y = new_pos.y as usize;

        let mut res = old_pos;

        // Going along x won't cause a collision.
        if self.grid[self.wd * old_idx_y + new_idx_x] == 0 {
            res.x = new_pos.x;
        }

        // Going along y won't cause a collision.
        if self.grid[self.wd * new_idx_y + old_idx_x] == 0 {
            res.y = new_pos.y;
        }

        res
    }

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
                        tex: tex - 1,
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
                        tex: tex - 1,
                        in_ns_dir: true,
                    };
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [u8; 36] = [
        1, 1, 1, 1, 1, 1, // row-0
        1, 0, 0, 0, 0, 1, // row-1
        1, 0, 0, 0, 0, 1, // row-2
        1, 0, 0, 0, 0, 1, // row-3
        1, 0, 0, 0, 0, 1, // row-4
        1, 1, 1, 1, 1, 1, // row-5
    ];

    fn make_map() -> Map {
        Map {
            wd: 6,
            ht: 6,
            grid: GRID.into(),
        }
    }

    #[test]
    fn resolve_collisions_given_bad_new_x() {
        let map = make_map();

        let old_pos = Vec2::new(4.8, 4.8);
        let new_pos = Vec2::new(5.1, 4.9);
        assert_eq!(map.resolve_collisions(old_pos, new_pos), Vec2::new(4.8, 4.9));

        let old_pos = Vec2::new(1.2, 1.2);
        let new_pos = Vec2::new(0.9, 1.1);
        assert_eq!(map.resolve_collisions(old_pos, new_pos), Vec2::new(1.2, 1.1));
    }

    #[test]
    fn resolve_collisions_given_bad_new_y() {
        let map = make_map();
        let old_pos = Vec2::new(4.8, 4.8);
        let new_pos = Vec2::new(4.9, 5.1);
        
        assert_eq!(map.resolve_collisions(old_pos, new_pos), Vec2::new(4.9, 4.8));

        let old_pos = Vec2::new(1.2, 1.2);
        let new_pos = Vec2::new(1.1, 0.9);
        assert_eq!(map.resolve_collisions(old_pos, new_pos), Vec2::new(1.1, 1.2));
    }

    #[test]
    fn intersect_given_horizontal_ray_works_fine() {
        let map = make_map();
        let ray = Ray2::new(Vec2::new(1.5, 1.5), Vec2::new(1.0, 0.0));
        let intersection = map.intersect(&ray);

        assert_eq!(intersection.pos, Vec2::new(5.0, 1.5));
        assert_eq!(intersection.tex, 0);
        assert!(intersection.in_ns_dir);
    }

    #[test]
    fn intersect_recognizes_ew_intersection() {
        let map = make_map();
        let ray = Ray2::new(Vec2::new(2.0, 2.0), Vec2::new(1.0, 2.0));
        let intersection = map.intersect(&ray);

        assert!(!intersection.in_ns_dir);
    }

    #[test]
    fn intersect_recognizes_ns_intersection() {
        let map = make_map();
        let ray = Ray2::new(Vec2::new(2.0, 2.0), Vec2::new(2.0, 1.0));
        let intersection = map.intersect(&ray);

        assert!(intersection.in_ns_dir);
    }
}
