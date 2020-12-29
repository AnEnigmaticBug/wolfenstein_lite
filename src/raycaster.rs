use crate::{camera::Camera, map::Map};

pub struct Raycaster {
    pub scr_wd: u32,
    pub scr_ht: u32,
}

impl Raycaster {
    pub fn render(&self, camera: &Camera, map: &Map, buf: &mut [u8]) {
        let scr_wd = self.scr_wd as f32;
        let scr_ht = self.scr_ht as f32;

        for x in 0..self.scr_wd {
            let pct = 2.0 * (x as f32 - scr_wd / 2.0) / scr_wd;
            let ray = camera.ray(pct);
            let intersection = map.intersect(&ray);

            let color = if intersection.in_ns_dir {
                [64; 3]
            } else {
                [96; 3]
            };
            let white = [224; 3];

            let cos = ray.dir.dot(&camera.dir);
            let perp_dist = (intersection.pos - camera.pos).len() * cos;
            let wall_ht = 1.0 * scr_ht / perp_dist;
            let offs = ((scr_ht - wall_ht) / 2.0).max(0.0);

            let wall_top = offs as u32;
            let wall_bot = (scr_ht - offs) as u32;

            for y in 0..self.scr_ht {
                let i = 4 * (self.scr_wd * y + x) as usize;
                if wall_top < y && y < wall_bot {
                    buf[i + 0] = color[0];
                    buf[i + 1] = color[1];
                    buf[i + 2] = color[2];
                    buf[i + 3] = 255;
                } else {
                    buf[i + 0] = white[0];
                    buf[i + 1] = white[1];
                    buf[i + 2] = white[2];
                    buf[i + 3] = 255;
                }
            }
        }
    }
}
