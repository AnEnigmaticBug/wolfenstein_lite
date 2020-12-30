use std::f32::consts::PI;

use pixels::{Pixels, SurfaceTexture};
use raycaster::{
    camera::Camera, map::Map, primitive::Vec2, raycaster::Raycaster, texture::Texture,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const SCR_WD: u32 = 640;
const SCR_HT: u32 = 480;
const MAP_WD: usize = 24;
const MAP_HT: usize = 24;
const GRID: [u8; MAP_WD * MAP_HT] = [
    8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 8, 4, 4, 6, 4, 4, 6, 4, 6, 4, 4, 4, 6, 4, //
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, //
    8, 0, 3, 3, 0, 0, 0, 0, 0, 8, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, //
    8, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, //
    8, 0, 3, 3, 0, 0, 0, 0, 0, 8, 8, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, //
    8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 4, 0, 0, 0, 0, 0, 6, 6, 6, 0, 6, 4, 6, //
    8, 8, 8, 8, 0, 8, 8, 8, 8, 8, 8, 4, 4, 4, 4, 4, 4, 6, 0, 0, 0, 0, 0, 6, //
    7, 7, 7, 7, 0, 7, 7, 7, 7, 0, 8, 0, 8, 0, 8, 0, 8, 4, 0, 4, 0, 6, 0, 6, //
    7, 7, 0, 0, 0, 0, 0, 0, 7, 8, 0, 8, 0, 8, 0, 8, 8, 6, 0, 0, 0, 0, 0, 6, //
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 6, 0, 0, 0, 0, 0, 4, //
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 6, 0, 6, 0, 6, 0, 6, //
    7, 7, 0, 0, 0, 0, 0, 0, 7, 8, 0, 8, 0, 8, 0, 8, 8, 6, 4, 6, 0, 6, 6, 6, //
    7, 7, 7, 7, 0, 7, 7, 7, 7, 8, 8, 4, 0, 6, 8, 4, 8, 3, 3, 3, 0, 3, 3, 3, //
    2, 2, 2, 2, 0, 2, 2, 2, 2, 4, 6, 4, 0, 0, 6, 0, 6, 3, 0, 0, 0, 0, 0, 3, //
    2, 2, 0, 0, 0, 0, 0, 2, 2, 4, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 3, //
    2, 0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 4, 3, 0, 0, 0, 0, 0, 3, //
    1, 0, 0, 0, 0, 0, 0, 0, 1, 4, 4, 4, 4, 4, 6, 0, 6, 3, 3, 0, 0, 0, 3, 3, //
    2, 0, 0, 0, 0, 0, 0, 0, 2, 2, 2, 1, 2, 2, 2, 6, 6, 0, 0, 5, 0, 5, 0, 5, //
    2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 5, 0, 5, 0, 0, 0, 5, 5, //
    2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 5, 0, 5, 0, 5, 0, 5, 0, 5, //
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, //
    2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 5, 0, 5, 0, 5, 0, 5, 0, 5, //
    2, 2, 0, 0, 0, 0, 0, 2, 2, 2, 0, 0, 0, 2, 2, 0, 5, 0, 5, 0, 0, 0, 5, 5, //
    2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 5, 5, 5, 5, 5, 5, 5, 5, 5, //
];
const SPEED: f32 = 0.08;

fn main() {
    let main_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("Raycaster")
        .with_inner_size(LogicalSize::new(SCR_WD as f64, SCR_HT as f64))
        .with_resizable(false)
        .build(&main_loop)
        .unwrap();

    let surf = SurfaceTexture::new(SCR_WD, SCR_HT, &window);
    let mut pixels = Pixels::new(SCR_WD, SCR_HT, surf).unwrap();

    let mut camera = Camera::new(Vec2::new(5.0, 3.0), Vec2::new(1.0, 0.0), 90.0);
    let map = Map {
        wd: MAP_WD,
        ht: MAP_HT,
        grid: GRID.into(),
    };
    let caster = Raycaster {
        scr_wd: SCR_WD,
        scr_ht: SCR_HT,
        textures: load_textures(),
        floor_id: 3,
    };

    main_loop.run(move |event, _, cflow| {
        *cflow = ControlFlow::Wait;

        if let Event::RedrawRequested(_) = event {
            caster.render(&camera, &map, pixels.get_frame());
            pixels.render().unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *cflow = ControlFlow::Exit;
                return;
            }

            let mut moved = false;

            if input.key_held(VirtualKeyCode::Left) {
                camera.rotate_by(-SPEED);
                moved = true;
            }

            if input.key_held(VirtualKeyCode::Right) {
                camera.rotate_by(SPEED);
                moved = true;
            }

            let mut offs = Vec2::new(0.0, 0.0);

            if input.key_held(VirtualKeyCode::W) {
                offs = offs + camera.dir * SPEED;
            }

            if input.key_held(VirtualKeyCode::S) {
                offs = offs - camera.dir * SPEED;
            }

            if input.key_held(VirtualKeyCode::A) {
                offs = offs - camera.dir.rotated(PI / 2.0) * SPEED;
            }
            if input.key_held(VirtualKeyCode::D) {
                offs = offs + camera.dir.rotated(PI / 2.0) * SPEED;
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

fn load_textures() -> Vec<Texture> {
    [
        "res/eagle.png",
        "res/red_brick.png",
        "res/purple_stone.png",
        "res/grey_stone.png",
        "res/blue_stone.png",
        "res/moss_stone.png",
        "res/wood.png",
        "res/color_stone.png",
    ]
    .iter()
    .map(|path| Texture::load(path).unwrap())
    .collect()
}
