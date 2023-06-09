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
        device::{
            physical::{PhysicalDevice, PhysicalDeviceType}, DeviceExtensions,
            QueueFlags,
        },
    };

    pub struct Application {
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        physical_device: Arc<PhysicalDevice>,
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

            let physical_device = Self::choose_physical_device(&instance, &surface);

            Application {
                instance,
                surface,
                physical_device,
                event_loop,
            }
        }

        pub fn create_instance() -> Arc<Instance> {
            let library = VulkanLibrary::new().unwrap();

            let required_extensions = vulkano_win::required_extensions(&library);

            Instance::new(
                library,
                InstanceCreateInfo {
                    enabled_extensions: required_extensions,
                    engine_name: Some("Halvar".into()),
                    enumerate_portability: true,
                    ..Default::default()
                },
            )
            .unwrap()
        }

        pub fn choose_physical_device(instance: &Arc<Instance>, surface: &Arc<Surface>) -> Arc<PhysicalDevice> {
            let device_extensions = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::empty()
            };

            let (physical_device, _queue_family_index) = instance
                .enumerate_physical_devices()
                .unwrap()
                .filter(|p| {
                    p.supported_extensions().contains(&device_extensions)
                })
                .filter_map(|p| {
                    p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
                })
                .min_by_key(|(p, _)| {
                    match p.properties().device_type {
                        PhysicalDeviceType::DiscreteGpu => 0,
                        PhysicalDeviceType::IntegratedGpu => 1,
                        PhysicalDeviceType::VirtualGpu => 2,
                        PhysicalDeviceType::Cpu => 3,
                        PhysicalDeviceType::Other => 4,
                        _ => 5,
                    }
                })
                .expect("No suitable physical device found");

            // Some little debug infos.
            println!(
                "Using device: {} (type: {:?})",
                physical_device.properties().device_name,
                physical_device.properties().device_type,
            );

            physical_device
        }

        pub fn run(self) {
            if Arc::strong_count(&self.instance) == 0 {
                println!("Cannot run application without a vulkan instance");
                return;
            }
            if Arc::strong_count(&self.physical_device) == 0 {
                println!("Cannot run application without a physical rendering device");
                return;
            }
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
                        if dimensions.width == 0 || dimensions.height == 0 {}
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
