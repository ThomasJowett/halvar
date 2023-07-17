pub mod halvar {
    use std::sync::Arc;
    use vulkano::{
        device::{
            physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
            QueueCreateInfo, QueueFlags,
        },
        image::{ImageUsage, SwapchainImage},
        instance::{Instance, InstanceCreateInfo},
        swapchain::{Surface, Swapchain, SwapchainCreateInfo},
        VulkanLibrary,
    };
    use vulkano_win::VkSurfaceBuild;
    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        window::{Window, WindowBuilder},
    };

    pub struct Application {
        instance: Arc<Instance>,
        surface: Arc<Surface>,
        device: Arc<Device>,
        queue: Arc<Queue>,
        swapchain: Arc<Swapchain>,
        images: Vec<Arc<SwapchainImage>>,
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

            let (device, queue) = Self::create_device(&instance, &surface);

            let (swapchain, images) = Self::create_swapchain(&device, &surface);

            Application {
                instance,
                surface,
                device,
                queue,
                swapchain,
                images,
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

        pub fn create_device(
            instance: &Arc<Instance>,
            surface: &Arc<Surface>,
        ) -> (Arc<Device>, Arc<Queue>) {
            let device_extensions = DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::empty()
            };

            let (physical_device, queue_family_index) = instance
                .enumerate_physical_devices()
                .unwrap()
                .filter(|p| p.supported_extensions().contains(&device_extensions))
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
                .min_by_key(|(p, _)| match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                })
                .expect("No suitable physical device found");

            // Some little debug infos.
            println!(
                "Using device: {} (type: {:?})",
                physical_device.properties().device_name,
                physical_device.properties().device_type,
            );

            let (device, mut queues) = Device::new(
                physical_device,
                DeviceCreateInfo {
                    enabled_extensions: device_extensions,
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }],

                    ..Default::default()
                },
            )
            .unwrap();

            let queue: Arc<vulkano::device::Queue> = queues.next().unwrap();

            (device, queue)
        }

        pub fn create_swapchain(
            device: &Arc<Device>,
            surface: &Arc<Surface>,
        ) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
            let surface_capabilities = device
                .physical_device()
                .surface_capabilities(surface, Default::default())
                .unwrap();

            let image_format = Some(
                device
                    .physical_device()
                    .surface_formats(surface, Default::default())
                    .unwrap()[0]
                    .0,
            );

            let window = surface.object().unwrap().downcast_ref::<Window>().unwrap();

            Swapchain::new(
                device.clone(),
                surface.clone(),
                SwapchainCreateInfo {
                    min_image_count: surface_capabilities.min_image_count,
                    image_format,
                    image_extent: window.inner_size().into(),
                    image_usage: ImageUsage::COLOR_ATTACHMENT,
                    composite_alpha: surface_capabilities
                        .supported_composite_alpha
                        .into_iter()
                        .next()
                        .unwrap(),
                    ..Default::default()
                },
            )
            .unwrap()
        }

        pub fn run(mut self) {
            if Arc::strong_count(&self.instance) == 0 {
                println!("Cannot run application without a vulkan instance");
                return;
            }
            if Arc::strong_count(&self.device) == 0 {
                println!("Cannot run application without a vulkan device");
                return;
            }
            if Arc::strong_count(&self.queue) == 0 {
                println!("Cannot run application without a vulkan queue");
                return;
            }
            if Arc::strong_count(&self.surface) == 0 {
                println!("Cannot run application without a vulkan surface");
                return;
            }
            if Arc::strong_count(&self.swapchain) == 0 {
                println!("Cannot run application without a vulkan swapchain");
                return;
            }
            if Vec::is_empty(&self.images) {
                println!("Cannot run application without vulkan images");
                return;
            }

            let mut recreate_swapchain = false;

            self.event_loop
                .run(move |event, _, control_flow| match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        ..
                    } => control_flow.set_exit(),
                    Event::WindowEvent { 
                        event: WindowEvent::Resized(_),
                        ..
                    } => {
                        recreate_swapchain = true;
                    }
                    Event::RedrawEventsCleared => {
                        let window = self
                            .surface
                            .object()
                            .unwrap()
                            .downcast_ref::<Window>()
                            .unwrap();
                        let image_extent: [u32; 2] = window.inner_size().into();

                        // If the window size is zero then don't draw anything
                        if image_extent.contains(&0) { return; }

                        if recreate_swapchain {
                            let (new_swapchain, _new_images) = self.swapchain
                                .recreate(SwapchainCreateInfo {
                                    image_extent,
                                    ..self.swapchain.create_info()
                                })
                                .expect("Failed to recreate swapchain");

                            self.swapchain = new_swapchain;
                            recreate_swapchain = false;
                        }
                    }
                    _ => (),
                });
        }
    }
}