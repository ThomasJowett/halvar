extern crate vulkano;
extern crate vulkano_win;
extern crate winit;

mod halvar {
    use vulkano_win::VkSurfaceBuild;
    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::{Window, WindowBuilder},
    };

    use std::sync::Arc;
    use vulkano::{
        instance::{Instance, InstanceCreateInfo},
        swapchain::Surface,
        VulkanLibrary,
    };

    pub struct Application {
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        event_loop: EventLoop<()>,
    }

    impl Application {
        pub fn new() -> Self {
            let instance = Self::create_instance();
            let event_loop = EventLoop::new();
            let surface = WindowBuilder::new()
                .with_title("Halvar")
                .with_inner_size(LogicalSize::new(800, 600))
                .build_vk_surface(&event_loop, instance.clone())
                .unwrap();
            Application {
                instance,
                surface,
                event_loop,
            }
        }

        pub fn create_instance() -> Arc<Instance> {
            let library = VulkanLibrary::new().unwrap();

            let required_extensions = vulkano_win::required_extensions(&library);

            let instance = Instance::new(
                library,
                InstanceCreateInfo {
                    enabled_extensions: required_extensions,
                    engine_name: Some("Halvar".into()),
                    enumerate_portability: true,
                    ..Default::default()
                },
            )
            .unwrap();

            return instance;
        }

        pub fn run(self) {
            self.event_loop
                .run(move |event, _, control_flow| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => control_flow.set_exit(),
                    Event::RedrawEventsCleared => {
                        let window = self
                            .surface
                            .object()
                            .unwrap()
                            .downcast_ref::<Window>()
                            .unwrap();
                        let dimensions = window.inner_size();
                        if dimensions.width == 0 || dimensions.height == 0 {
                            return;
                        }
                    }
                    _ => (),
                });
        }
    }
}

fn main() {
    let app = halvar::Application::new();
    app.run();
}
