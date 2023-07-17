extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod application;
mod vertex;

fn main() {
    use vertex::halvar::StandardVertex;

    let app = application::halvar::Application::new();

    let _vertices = [
        StandardVertex {
            position: [-0.5, -0.25],
        },
        StandardVertex {
            position: [0.0, 0.5],
        },
        StandardVertex {
            position: [0.25, -0.1],
        },
    ];
    app.run();
}