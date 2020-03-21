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
mod consts;
mod engine;
mod ui;

use gameplay::gamestate::{
    GameStateManager,
    main_menu::MainMenuGameState,
};

fn main()
{
    log::info("Starting Space War Supreme!");

    let eventloop = EventLoop::new();
    let mut engine = engine::Engine::new(&eventloop);
    let mut game_state_manager = GameStateManager::new(
        &mut engine,
        Box::new(MainMenuGameState::new())
    );

    eventloop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == engine.renderer.get_window().id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { event, .. } => engine.input.handle_window_event(&event),
            Event::DeviceEvent { event, .. } => engine.input.handle_device_event(&event),
            Event::MainEventsCleared => {
                game_state_manager.tick(&mut engine);
                if game_state_manager.should_exit {
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
