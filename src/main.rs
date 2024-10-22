// Hide the console window in windows+release mode
#![cfg_attr(target_os = "windows", 
   cfg_attr(not(debug_assertions), 
   windows_subsystem = "windows"))]

// External Dependencies
extern crate glium;
#[macro_use]
extern crate rental;
extern crate specs;
extern crate nalgebra;
extern crate itertools;
extern crate rhai;
extern crate rand;
extern crate image;
extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate sanitize_filename;
extern crate base64;
extern crate anyhow;
extern crate crossbeam_channel;

mod gameplay;
mod engine;
use gameplay::levels::spacewar;

use winit::{
    event_loop::ControlFlow,
    event::{Event, WindowEvent},
};
use engine::prelude::*;

fn main()
{
    log::info("Starting Space War Supreme!");

    let eventloop = glium::glutin::event_loop::EventLoop::new();
    let mut engine = engine::Engine::new(
        &eventloop, 
        Box::new(spacewar::SpaceWarLevel::new())
    );
    // TODO expand this to include all monitor names+resolutions
    println!("{:?}", engine.renderer.get_supported_resolutions());

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == engine.renderer.get_display().gl_window().window().id() => *control_flow = ControlFlow::Exit,
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

                engine.renderer.get_display().gl_window().window().request_redraw();
            },
            Event::RedrawRequested(_window_id) => {
                engine.draw_frame();
            },
            _ => (),
        }
    });
}
