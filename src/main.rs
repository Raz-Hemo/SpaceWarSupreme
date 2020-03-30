// Hide the console window in windows+release mode
#![cfg_attr(target_os = "windows", 
   cfg_attr(not(debug_assertions), 
   windows_subsystem = "windows"))]

use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent},
};

mod log;
mod gameplay;
use gameplay::levels::{Level, spacewar};
mod scripting;
mod utils;
mod consts;
mod engine;
mod ui;

fn main()
{
    log::info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let mut engine = engine::Engine::new(&eventloop);
    spacewar::SpaceWarLevel::new().load_level(&mut engine);

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == engine.renderer.get_window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => {
                engine.input.handle_window_event(&event,
                    engine.cfg.resolution_x,
                    engine.cfg.resolution_y
                );
            },
            Event::DeviceEvent { event, .. } => engine.input.handle_device_event(&event),
            Event::MainEventsCleared => {
                if let engine::TickResult::Exit = engine.tick() {
                    *control_flow = ControlFlow::Exit;
                }

                engine.renderer.get_window().request_redraw();
            },
            Event::RedrawRequested(_window_id) => {
                engine.draw_frame();
            },
            _ => (),
        }
    });
}
