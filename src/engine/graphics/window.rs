use crate::engine::prelude::*;
use glium::glutin::{ContextBuilder, window::{WindowBuilder, Icon}};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
};

pub fn make_window(eventloop: &EventLoop<()>) -> glium::Display {
    // Load the icon
    use image::GenericImageView;
    let icon = match utils::load_image(consts::ICON_PATH) {
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
    
    glium::Display::new(
        WindowBuilder::new()
        .with_title(consts::WINDOW_NAME)
        .with_inner_size(LogicalSize::new(
            consts::DEFAULT_RESOLUTION[0], 
            consts::DEFAULT_RESOLUTION[1]))
        .with_window_icon(icon)
        .with_resizable(false),
        ContextBuilder::new().with_depth_buffer(24),
        &eventloop
    ).expect("Failed to create window and OpenGL display")
}
