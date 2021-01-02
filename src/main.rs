#[macro_use]
extern crate log;

use std::{error::Error, f32::consts::PI, fmt, path::Path};

use native_dialog::{MessageDialog, MessageType};
use pixels::{Pixels, SurfaceTexture};
use raycaster::{
    camera::Camera, config::Config, map::Map, primitive::Vec2, raycaster::Raycaster,
    texture::Texture,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    env_logger::init();

    let config = Config::read("Config.toml")
        .map_err(|e| error_dialog(e, "Couldn't read Config.toml"))
        .unwrap();

    let scr_wd = config.screen.wd;
    let scr_ht = config.screen.ht;
    let player = config.player;
    let wall_ht_scale = config.misc.wall_ht_scale.unwrap_or(1.0);

    let main_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Raycaster")
        .with_inner_size(LogicalSize::new(scr_wd as f64, scr_ht as f64))
        .with_resizable(false)
        .build(&main_loop)
        .map_err(|e| error_dialog(e, "Couldn't create window"))
        .unwrap();

    debug!("Setup window");

    let surf = SurfaceTexture::new(scr_wd, scr_ht, &window);
    let mut pixels = Pixels::new(scr_wd, scr_ht, surf)
        .map_err(|e| error_dialog(e, "Couldn't setup drawing"))
        .unwrap();

    debug!("Setup pixels");

    let mut camera = Camera::new(
        player.initial_pos,
        player.initial_dir,
        player.fov.unwrap_or(90.0),
    );
    let map = Map::load(config.assets.map)
        .map_err(|e| error_dialog(e, "Couldn't load map file"))
        .unwrap();
    let caster = Raycaster {
        scr_wd,
        scr_ht,
        textures: load_textures(&config.assets.tex),
        floor_id: config.misc.floor_tex,
        wall_ht_scale,
    };

    debug!("Ready to run");

    main_loop.run(move |event, _, cflow| {
        // Only run the loop when an event occurs. The only reason why anything
        // should change is that the player moved i.e an event occured.
        *cflow = ControlFlow::Wait;

        if let Event::RedrawRequested(_) = event {
            caster.render(&camera, &map, pixels.get_frame());
            pixels
                .render()
                .map_err(|e| error_dialog(e, "Couldn't draw frame"))
                .unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                info!("Quit event received");
                *cflow = ControlFlow::Exit;
                return;
            }

            let mut moved = false;

            if input.key_held(VirtualKeyCode::Left) {
                camera.rotate_by(-player.speed);
                moved = true;
            }

            if input.key_held(VirtualKeyCode::Right) {
                camera.rotate_by(player.speed);
                moved = true;
            }

            let mut offs = Vec2::new(0.0, 0.0);

            if input.key_held(VirtualKeyCode::W) {
                offs += camera.dir * player.speed;
            }

            if input.key_held(VirtualKeyCode::S) {
                offs -= camera.dir * player.speed;
            }

            if input.key_held(VirtualKeyCode::A) {
                offs -= camera.dir.rotated(PI / 2.0) * player.speed;
            }
            if input.key_held(VirtualKeyCode::D) {
                offs += camera.dir.rotated(PI / 2.0) * player.speed;
            }

            if offs.len_squared() > 0.0 {
                camera.pos = map.resolve_collisions(camera.pos, camera.pos + offs);
                moved = true;
            }

            if moved {
                window.request_redraw();
            }
        }
    });
}

fn load_textures<P: AsRef<Path> + fmt::Debug>(paths: &[P]) -> Vec<Texture> {
    paths
        .iter()
        .map(|path| {
            Texture::load(path)
                .map_err(|e| error_dialog(e, "Couldn't load texture"))
                .unwrap()
        })
        .collect()
}

fn error_dialog<T: Error + Sized>(e: T, title: &str) -> T {
    error!("{}: {}", title, e);

    MessageDialog::new()
        .set_type(MessageType::Error)
        .set_title(title)
        .set_text(&format!("{}!", e))
        .show_alert()
        .expect("Couldn't show dialog");
    e
}
