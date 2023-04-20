extern crate vulkano;
extern crate winit;

mod halvar {
    use winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::{WindowBuilder, Window},
        dpi::LogicalSize
    };

    pub struct Application {
        window: Window,
        event_loop: EventLoop<()>
    }

    impl Application {
        pub fn new()->Self {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_title("Halvar")
                .with_inner_size(LogicalSize::new(800, 600))
                .build(&event_loop)
                .unwrap();
            Application { window, event_loop }
        }
    
        pub fn run(self) {
    
            self.event_loop.run(move |event, _, control_flow| {
                    println!("{event:?}");
    
                    match event {
                        Event::WindowEvent {
                            event: WindowEvent::CloseRequested,
                            window_id
                        } if window_id == self.window.id() => control_flow.set_exit(),
                        Event::MainEventsCleared => {
                            self.window.request_redraw();
                        }
                        _=> (),
                    }
                    
                });
            
        }
    }
}

fn main() {
    let app = halvar::Application::new();
    app.run();
}
