mod log;
mod gameplay;
mod scripting;
mod utils;
mod graphics;
mod config;
pub mod consts;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent},
};

fn main()
{
    log::logger().info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let window = graphics::window::make_window(&eventloop).unwrap();

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
