use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const SCR_WD: u32 = 640;
const SCR_HT: u32 = 480;

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

    main_loop.run(move |event, _, cflow| {
        if let Event::RedrawRequested(_) = event {
            draw_red_stripe(pixels.get_frame());
            pixels.render().unwrap();
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *cflow = ControlFlow::Exit;
                return;
            }
        }
    });
}

fn draw_red_stripe(buf: &mut [u8]) {
    for x in 200..240 {
        for y in 0..SCR_HT {
            let i = 4 * (SCR_WD * y + x) as usize;
            buf[i + 0] = 255;
            buf[i + 1] = 0;
            buf[i + 2] = 0;
            buf[i + 3] = 255;
        }
    }
}
