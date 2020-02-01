use winit::{
    dpi::LogicalSize,
    window::{WindowBuilder, Window},
    event_loop::EventLoop
};

pub fn make_window(eventloop: &EventLoop<()>) -> Window {
    WindowBuilder::new()
    .with_title(crate::consts::WINDOW_NAME)
    .with_inner_size(LogicalSize::new(400.0, 200.0))
    .build(&eventloop)
    .unwrap()
}
