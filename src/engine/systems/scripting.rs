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
    engine: rhai::Engine<'static>,
    loaded_scripts: HashMap<String, rhai::AST>,
    bad_scripts: HashSet<String>,
}

impl ScriptingSystem {
    pub fn new() -> ScriptingSystem {
        ScriptingSystem {
            events: Vec::new(),
            engine: scripting::new_engine(),
            loaded_scripts: HashMap::new(),
            bad_scripts: HashSet::new(),
        }
    }
    pub fn add_script(&mut self, path: &str) -> bool {
        match self.engine.compile_file(
            std::path::PathBuf::from("./scripts/").join(&sanitize_filename::sanitize(path))) {
            Ok(ast) => {
                self.loaded_scripts.insert(String::from(path), ast);
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
            let ast = match self.loaded_scripts.get_mut(&script.path) {
                Some(ast) => ast,
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
                    println!("{:?}", self.engine.call_fn0::<i64>(
                        &mut script.scope, 
                        ast,
                        "on_lclick"
                    ));
                }
            }
        }
    }
}