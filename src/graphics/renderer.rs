extern crate vulkano;

use std::sync::Arc;

use vulkano_win;
use winit::{
    window::Window,
    event_loop::EventLoop,
}; 
use vulkano::sync::SharingMode;
use vulkano::swapchain::{Swapchain, SurfaceTransform, Surface, PresentMode, CompositeAlpha,
                         FullscreenExclusive, ColorSpace};
use vulkano::image::swapchain::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features, Queue};

pub struct Renderer {
    surface: Arc<Surface<Window>>,
    swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>,
    instance: Arc<Instance>,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl Renderer {
    pub fn get_window(self: &Renderer) -> &Window {
        self.surface.window()
    }
}

fn pick_best_physical_device(inst: &Arc<Instance>) -> PhysicalDevice {
    // Start with any GPU
    let mut result = PhysicalDevice::enumerate(inst).next().expect("Failed to find a physical graphics device");

    // Discrete GPU is better
    for pd in PhysicalDevice::enumerate(inst) {
        if pd.ty() == PhysicalDeviceType::DiscreteGpu {
            result = pd
        }
    }

    crate::log::info(&format!("Using GPU '{}'", result.name()));
    result
}

impl Renderer {
    pub fn new(eventloop: &EventLoop<()>) -> Renderer {
        // Create instance + window
        let instance = Instance::new(
            None,
            &vulkano_win::required_extensions(),
            None).expect("Failed to create Vulkan instance");
        let surface = crate::graphics::window::make_window(eventloop, instance.clone());
        
        // Pick a physical device (i.e. GPU)
        let physical_device = pick_best_physical_device(&instance);
        let qf = physical_device.queue_families()
            .find(|&q| q.supports_graphics())
            .expect("Couldn't find a Vulkan queue family with graphics support");

        // Create device + command queues
        let (device, mut queues) = {
            Device::new(
                physical_device, 
                &Features::none(), 
                &DeviceExtensions {
                    khr_swapchain: true,
                    .. DeviceExtensions::none()
                },
                [(qf, 0.5)].iter().cloned()
            ).expect("Failed to create Vulkan device")
        };
        let queue = queues.next().expect("Device::new returned no queues");

        // Create swap chain
        let caps = surface.capabilities(physical_device)
            .expect("failed to get surface capabilities");

        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count, // number of buffers
            caps.supported_formats[0].0,
            caps.current_extent.unwrap_or([640, 480]),
            1, // layers of each buffer
            caps.supported_usage_flags,
            SharingMode::Exclusive,
            SurfaceTransform::Identity,
            CompositeAlpha::Opaque,
            PresentMode::Fifo,
            FullscreenExclusive::Disallowed,
            true, // Clip portions of the window that are outside the screen for performance
            ColorSpace::SrgbNonLinear
        ).expect("failed to create swapchain");


        Renderer {
            instance,
            surface,
            device, 
            queue,
            swapchain,
            swapchain_images: images,
        }
    }

    pub fn acquire(self: &Renderer) {
        let (image_num, suboptimal, acquire_future) =
            vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
    }
}