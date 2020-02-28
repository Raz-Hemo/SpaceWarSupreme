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
mod scripting;
mod utils;
mod graphics;
mod config;
mod consts;
mod input;
mod engine;
mod ui;
mod audio;

use gameplay::gamestate::{
    GameStateManager,
    main_menu::MainMenuGameState,
};

fn main()
{
    log::info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let renderer = graphics::renderer::Renderer::new(&eventloop);
    let mut engine = engine::Engine::new();
    let mut game_state_manager = GameStateManager::new(
        &mut engine,
        Box::new(MainMenuGameState::new())
    );
    engine.input.add_handler("A", Box::from(|e: &engine::Engine| {e.play_sound("btn_click");}));

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == renderer.get_window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => input::handle_event(&mut engine, &event),
            Event::DeviceEvent { event, .. } => input::handle_device_event(&mut engine.input, &event),
            Event::MainEventsCleared => {
                game_state_manager.tick(&mut engine);
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
