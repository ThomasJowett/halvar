extern crate sdl2;
extern crate gl;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Halvar", 900, 700)
        .opengl()
        .resizable()
        .position_centered()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }
    }
}
