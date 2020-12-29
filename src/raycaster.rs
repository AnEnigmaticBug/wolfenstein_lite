use crate::texture::Texture;
use crate::{camera::Camera, map::Map};

pub struct Raycaster {
    pub scr_wd: u32,
    pub scr_ht: u32,
    pub textures: Vec<Texture>,
}

impl Raycaster {
    pub fn render(&self, camera: &Camera, map: &Map, buf: &mut [u8]) {
        let scr_wd = self.scr_wd as f32;
        let scr_ht = self.scr_ht as f32;

        for x in 0..self.scr_wd {
            let pct = 2.0 * (x as f32 - scr_wd / 2.0) / scr_wd;
            let ray = camera.ray(pct);
            let intersection = map.intersect(&ray);

            let cos = ray.dir.dot(&camera.dir);
            let perp_dist = (intersection.pos - camera.pos).len() * cos;
            let wall_ht = 1.0 * scr_ht / perp_dist;
            let offs = ((scr_ht - wall_ht) / 2.0).max(0.0);

            let wall_top = offs as u32;
            let wall_bot = (scr_ht - offs) as u32;

            let tex = &self.textures[intersection.tex as usize];
            let tex_x = if intersection.in_ns_dir {
                (tex.wd as f32 * intersection.pos.y.fract()) as usize
            } else {
                (tex.wd as f32 * intersection.pos.x.fract()) as usize
            };

            let tex_darkness = if intersection.in_ns_dir { 2 } else { 1 };

            for y in 0..self.scr_ht {
                let i = 4 * (self.scr_wd * y + x) as usize;
                if wall_top < y && y < wall_bot {
                    let tex_y = (tex.ht as f32 * (y - wall_top) as f32 / wall_ht) as usize;
                    let tex_i = 3 * (tex.wd * tex_y + tex_x);

                    buf[i + 0] = tex.buf[tex_i + 0] / tex_darkness;
                    buf[i + 1] = tex.buf[tex_i + 1] / tex_darkness;
                    buf[i + 2] = tex.buf[tex_i + 2] / tex_darkness;
                    buf[i + 3] = 255;
                } else {
                    buf[i + 0] = 64;
                    buf[i + 1] = 64;
                    buf[i + 2] = 64;
                    buf[i + 3] = 255;
                }
            }
        }
    }
}
