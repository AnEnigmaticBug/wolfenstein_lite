mod reader;

use std::fmt;
use std::path::Path;

use crate::primitive::{Ray2, Vec2};
use reader::{read_map, MapReadError};

pub type TexId = u8;

/// Represents a map in which the player can run around.
pub struct Map {
    pub wd: usize,
    pub ht: usize,
    /// Each entry represents a cell in the map's grid. If it is a `None`, the
    /// cell is empty. If it is `Some(tex)`, then the cell has a wall having a
    /// texture of id `tex`.
    pub grid: Vec<Option<TexId>>,
}

/// Info about the point where a ray intersected a wall.
#[derive(Debug)]
pub struct Intersection {
    pub pos: Vec2,
    pub tex: TexId,
    pub in_ns_dir: bool,
}

impl Map {
    /// Loads a map from at `path`.
    pub fn load<P: AsRef<Path> + fmt::Debug>(path: P) -> Result<Self, MapReadError> {
        read_map(path)
    }

    /// Goes from `old_pos` to `new_pos` while staying out of walls.
    ///
    /// It works by simply ignoring the components of displacement which cause
    /// that issue. This allows us to retain the other harmless components. It
    /// allows us to slide along walls.
    pub fn resolve_collisions(&self, old_pos: Vec2, new_pos: Vec2) -> Vec2 {
        let old_idx_x = old_pos.x as usize;
        let old_idx_y = old_pos.y as usize;
        let new_idx_x = new_pos.x as usize;
        let new_idx_y = new_pos.y as usize;

        let mut res = old_pos;

        // Going along x won't cause a collision.
        if self.grid[self.wd * old_idx_y + new_idx_x].is_none() {
            res.x = new_pos.x;
        }

        // Going along y won't cause a collision.
        if self.grid[self.wd * new_idx_y + old_idx_x].is_none() {
            res.y = new_pos.y;
        }

        res
    }

    /// Calculates information about the point of intersection of `ray` with a
    /// wall in `self.map`.
    ///
    /// An intersection point always exists because all maps in this demo have
    /// to be closed i.e no ray can go on infinitely without hitting a wall :)
    ///
    /// # Overall idea
    ///
    /// Since all walls are aligned to a grid, intersection points will always
    /// have either an integral x-coordinate or an integral y-coordinate. This 
    /// means to find the intersection point, we just need to find the closest
    /// point along `ray.dir` which:
    /// * has integral x or y coordinates
    /// * is on a wall
    ///
    /// To find the closest point, we keep on generating successively far away
    /// integral x and y coordinate points till we hit a wall.
    ///
    /// How do we find out successive points?
    ///
    /// All points along `ray.dir` having integral x-coordinates have the same
    /// separation `step_ns`. Similarly all points with integral y-coordinates
    /// have a separation of `step_ew`. By the way, _ns_ means north-south and
    /// _ew_ means east-west. All points having integral x-coordinates hit the
    /// face of a wall parallel to the north-south axis).
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
            // The next EW point is closer than the next NS point.
            if (pos_ew.x * dir.x) < (pos_ns.x as f32 * dir.x) {
                let res = pos_ew;
                pos_ew += step_ew;

                let idx_x = res.x as usize;
                let idx_y = if dir.y > 0.0 { res.y } else { res.y - 1.0 } as usize;

                if let Some(tex) = self.grid[self.wd * idx_y + idx_x] {
                    return Intersection {
                        pos: res,
                        tex,
                        in_ns_dir: false,
                    };
                }
            } else {
                let res = pos_ns;
                pos_ns += step_ns;

                let idx_x = if dir.x > 0.0 { res.x } else { res.x - 1.0 } as usize;
                let idx_y = res.y as usize;

                if let Some(tex) = self.grid[self.wd * idx_y + idx_x] {
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

#[cfg(test)]
mod tests {
    use super::*;

    const GRID: [char; 36] = [
        '0', '0', '0', '0', '0', '0', // row-0
        '0', ' ', ' ', ' ', ' ', '0', // row-1
        '0', ' ', ' ', ' ', ' ', '0', // row-2
        '0', ' ', ' ', ' ', ' ', '0', // row-3
        '0', ' ', ' ', ' ', ' ', '0', // row-4
        '0', '0', '0', '0', '0', '0', // row-5
    ];

    fn make_map() -> Map {
        Map {
            wd: 6,
            ht: 6,
            grid: GRID
                .iter()
                .map(|&c| {
                    if c == ' ' {
                        None
                    } else {
                        Some(c.to_digit(16).unwrap() as u8)
                    }
                })
                .collect(),
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
