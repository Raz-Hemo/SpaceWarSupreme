mod log;
mod gameplay;
mod scripting;
mod utils;
mod graphics;
mod config;
mod consts;
mod input;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent},
};

fn main()
{
    log::logger().info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let window = graphics::window::make_window(&eventloop);
    let mut input_info = input::InputInfo::new();
    let mut renderer = graphics::renderer::Renderer::new();

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => input::handle_event(&mut input_info, &event),
            Event::DeviceEvent { event, .. } => input::handle_device_event(&mut input_info, &event),
            Event::MainEventsCleared => {
                // TODO do logic
                window.request_redraw();
            },
            Event::RedrawRequested(_window_id) => {
                // TODO draw frame
            },
            _ => (),
        }
    });
}
