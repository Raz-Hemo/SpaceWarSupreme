use winit::{
    dpi::LogicalSize,
    window::{WindowBuilder, Window, Icon},
    event_loop::EventLoop,
};
use crate::utils;

pub fn make_window(eventloop: &EventLoop<()>) -> utils::SWSResult<Window> {
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
    Ok(WindowBuilder::new()
    .with_title(crate::consts::WINDOW_NAME)
    .with_inner_size(LogicalSize::new(400.0, 200.0))
    .with_window_icon(icon)
    .build(&eventloop)
    .unwrap())
}
