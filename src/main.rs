// Hide the console window in windows+release mode
#![cfg_attr(target_os = "windows", 
   cfg_attr(not(debug_assertions), 
   windows_subsystem = "windows"))]

// External Dependencies
extern crate specs;
extern crate cgmath;
extern crate itertools;
extern crate vulkano;
extern crate vulkano_shaders;
extern crate rhai;
extern crate rand;
extern crate image;
extern crate chrono;
extern crate serde_json;

mod log;
mod gameplay;
use gameplay::levels::spacewar;
mod scripting;
mod utils;
mod consts;
mod engine;
mod ui;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    event::{Event, WindowEvent},
};

fn main()
{
    log::info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let mut engine = engine::Engine::new(
        &eventloop, 
        Box::new(spacewar::SpaceWarLevel::new())
    );

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == engine.renderer.get_window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => {
                engine.input.handle_window_event(
                    &event,
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
