use std::sync::Arc;

use vulkano::{
    swapchain::Surface,
    instance::Instance,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    dpi::LogicalSize,
    window::{WindowBuilder, Window, Icon},
    event_loop::EventLoop,
};

use crate::utils;

pub fn make_window(eventloop: &EventLoop<()>, instance: Arc<Instance>) -> Arc<Surface<Window>> {
    // Load the icon
    use image::GenericImageView;
    let icon = match utils::load_image(crate::consts::ICON_PATH) {
        Ok(img) => {
            let (width, height) = img.dimensions();
            let img = img.into_rgba().into_raw();
            match Icon::from_rgba(img, width, height) {
                Ok(i) => Some(i),
                Err(_) => None,
            }
        },
        Err(_) => None
    };

    // Build the window
    WindowBuilder::new()
        .with_title(crate::consts::WINDOW_NAME)
        .with_inner_size(LogicalSize::new(640, 480))
        .with_window_icon(icon)
        .build_vk_surface(&eventloop, instance)
        .expect("Failed to create window")
}
