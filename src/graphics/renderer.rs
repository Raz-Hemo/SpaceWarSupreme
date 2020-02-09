extern crate vulkano;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use std::sync::Arc;

pub struct Renderer {
    instance: Arc<Instance>,

}

impl Renderer {
    pub fn new() -> Renderer {
        let instance = Instance::new(
            None,
            &InstanceExtensions::none(),
            None).expect("Failed to create Vulkan instance");

        for physical_device in PhysicalDevice::enumerate(&instance) {
            println!("{:?}", physical_device);
        }

        Renderer {
            instance,
        }
    }
}