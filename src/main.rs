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

    let mut input_info = input::InputInfo::new();
    let eventloop = EventLoop::new();
    let renderer = graphics::renderer::Renderer::new(&eventloop);
    let mut game_state_manager = gameplay::gamestate::GameStateManager::new();

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.get_window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => input::handle_event(&mut input_info, &event),
            Event::DeviceEvent { event, .. } => input::handle_device_event(&mut input_info, &event),
            Event::MainEventsCleared => {
                game_state_manager.tick();
                if game_state_manager.should_exit {
                    *control_flow = ControlFlow::Exit;
                }

                renderer.get_window().request_redraw();
            },
            Event::RedrawRequested(_window_id) => {
                // TODO this is where rendering code sits
            },
            _ => (),
        }
    });
}
