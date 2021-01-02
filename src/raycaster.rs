use crate::texture::Texture;
use crate::{camera::Camera, map::Map};

pub struct Raycaster {
    pub scr_wd: u32,
    pub scr_ht: u32,
    pub textures: Vec<Texture>,
    /// Index of the `Texture` to be used for floor and roof.
    pub floor_id: usize,
    pub wall_ht_scale: f32,
}

impl Raycaster {
    /// Renders `map` from the POV of `camera` into `buf`.
    pub fn render(&self, camera: &Camera, map: &Map, buf: &mut [u8]) {
        assert_eq!(self.scr_wd * self.scr_ht * 4, buf.len() as u32);

        self.render_floor(camera, buf);
        self.render_walls(camera, map, buf);
    }

    /// Renders the floor and roof. It's also black magic.
    fn render_floor(&self, camera: &Camera, buf: &mut [u8]) {
        let scr_wd = self.scr_wd as f32;
        let scr_ht = self.scr_ht as f32;

        let tex = &self.textures[self.floor_id];

        for y in 0..self.scr_ht {
            let ray_ltmost = camera.ray(-1.0);
            let ray_rtmost = camera.ray(1.0);

            let cam_ht = scr_ht / 2.0;
            let p = y as f32 - cam_ht;
            let row_dist = cam_ht / p;

            let mut pos = camera.pos + row_dist * ray_ltmost.dir;
            let step = row_dist * (ray_rtmost.dir - ray_ltmost.dir) / scr_wd;

            for x in 0..self.scr_wd {
                let tex_x = (tex.wd as f32 * pos.x.fract()) as usize % tex.wd;
                let tex_y = (tex.ht as f32 * pos.y.fract()) as usize % tex.ht;

                pos = pos + step;

                let i = 4 * (self.scr_wd * y + x) as usize;
                let tex_i = 3 * (tex.wd * tex_y + tex_x);

                buf[i + 0] = tex.buf[tex_i + 0] / 3;
                buf[i + 1] = tex.buf[tex_i + 1] / 3;
                buf[i + 2] = tex.buf[tex_i + 2] / 3;
                buf[i + 3] = 255;

                let i = 4 * (self.scr_wd * (self.scr_ht - y - 1) + x) as usize;

                buf[i + 0] = tex.buf[tex_i + 0] / 2;
                buf[i + 1] = tex.buf[tex_i + 1] / 2;
                buf[i + 2] = tex.buf[tex_i + 2] / 2;
                buf[i + 3] = 255;
            }
        }
    }

    /// Renders all the walls. Walls along north-south axis have a darker tint
    /// to fake rudimentary lighting.
    ///
    /// # Overall idea
    ///
    /// For each column in the screen:
    /// 1. Cast a ray and note its intersection with a wall
    /// 2. Calculate the perpendicular distance of the intersection point
    /// 3. Greater the distance, smaller the wall's height for the column
    ///
    /// Camera is always at half-height of the scene. So, all walls have to be
    /// vertically centered.
    ///
    /// Each column of the screen corresponds to a wall and is drawn using its
    /// texture. 1 column of screen takes color from 1 column of texture.
    ///
    /// As textures tile horizontally every 1 unit, we just use the fractional
    /// part of intersection point's  non-integral co-ordinate to decide which
    /// texture column to use.
    fn render_walls(&self, camera: &Camera, map: &Map, buf: &mut [u8]) {
        let scr_wd = self.scr_wd as f32;
        let scr_ht = self.scr_ht as f32;

        for x in 0..self.scr_wd {
            let pct = 2.0 * (x as f32 - scr_wd / 2.0) / scr_wd;
            let ray = camera.ray(pct);
            let intersection = map.intersect(&ray);

            let cos = ray.dir.dot(&camera.dir);
            let perp_dist = (intersection.pos - camera.pos).len() * cos;
            let wall_ht = self.wall_ht_scale * scr_ht / perp_dist;
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
                if wall_top < y && y < wall_bot {
                    let i = 4 * (self.scr_wd * y + x) as usize;
                    let tex_y = (tex.ht as f32 * (y - wall_top) as f32 / wall_ht) as usize;
                    let tex_i = 3 * (tex.wd * tex_y + tex_x);

                    buf[i + 0] = tex.buf[tex_i + 0] / tex_darkness;
                    buf[i + 1] = tex.buf[tex_i + 1] / tex_darkness;
                    buf[i + 2] = tex.buf[tex_i + 2] / tex_darkness;
                    buf[i + 3] = 255;
                }
            }
        }
    }
}
