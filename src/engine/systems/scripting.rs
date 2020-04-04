use std::collections::{HashMap, HashSet};
use specs::WriteStorage;
use crate::engine::components::{ScriptingComponent, MouseComponent};
use crate::scripting;

pub enum GameEvent {
    ChangeResolution(u32, u32),
    ExitGame,
}

pub struct ScriptingSystem {
    pub events: Vec<GameEvent>,
    loaded_scripts: HashMap<String, rhai::Engine<'static>>,
    bad_scripts: HashSet<String>,
}

impl ScriptingSystem {
    pub fn new() -> ScriptingSystem {
        ScriptingSystem {
            events: Vec::new(),
            loaded_scripts: HashMap::new(),
            bad_scripts: HashSet::new(),
        }
    }
    pub fn add_script(&mut self, path: &str) -> bool {
        let mut engine = scripting::new_engine();
        match engine.consume_file(
            true,
            std::path::PathBuf::from("./scripts/").join(
                &sanitize_filename::sanitize(path))) {
            Ok(_) => {
                self.loaded_scripts.insert(String::from(path), engine);
                true
            },
            Err(e) => {
                crate::log::error(&format!("Failed to compile script {}: {}", path, e));
                self.bad_scripts.insert(String::from(path));
                false
            },
        }
    }
}

impl<'a> specs::System<'a> for ScriptingSystem {
    type SystemData = (
        WriteStorage<'a, ScriptingComponent>,
        WriteStorage<'a, MouseComponent>,
    );

    fn run(&mut self, (mut scripts, mut mouses): Self::SystemData) {
        use specs::Join;

        // First call the mouse functions (on_click, etc.)
        for (script, mouse) in (&mut scripts, (&mut mouses).maybe()).join() {
            // Best efford load of the script
            let mut engine = match self.loaded_scripts.get_mut(&script.path) {
                Some(engine) => engine,
                None => {
                    if !self.bad_scripts.contains(&script.path) {
                        if self.add_script(&script.path) {
                            self.loaded_scripts.get_mut(&script.path).unwrap()
                        } else {
                            continue;
                        }
                    } else {
                        continue;
                    }
                }
            };

            if let Some(mouse_some) = mouse {
                if mouse_some.is_clicked {
                    mouse_some.is_clicked = false;
                    //println!("{:?}", engine.eval_with_scope::<i32>(&mut script.scope, "on_lclick()"));
                    println!("{:?}", engine.call_fn::<_, i32>("on_lclick", (1337 as i32)));
                }
            }
        }
    }
}