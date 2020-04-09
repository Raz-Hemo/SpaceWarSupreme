use std::collections::{HashMap, HashSet};
use specs::WriteStorage;
use crate::engine::components::{ScriptingComponent, MouseComponent, KeyboardComponent};
use crate::scripting::{new_engine, GameContext};
use rhai::{Engine, Scope, AST};

pub struct ScriptingSystem {
    engine: Engine<'static>,
    scope: Scope<'static>,
    loaded_scripts: HashMap<String, AST>,
    bad_scripts: HashSet<String>,
}

impl ScriptingSystem {
    pub fn new() -> ScriptingSystem {
        let mut scope = Scope::new();
        scope.push("game", GameContext::new());
        ScriptingSystem {
            engine: new_engine(),
            scope,
            loaded_scripts: HashMap::new(),
            bad_scripts: HashSet::new(),
        }
    }

    pub fn get_game_context(&self) -> GameContext {
        self.scope.get_value::<GameContext>("game").unwrap()
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
        WriteStorage<'a, KeyboardComponent>,
    );

    fn run(&mut self, (mut scripts, mut mouses, mut keybs): Self::SystemData) {
        use specs::Join;

        // First call the mouse functions (on_click, etc.)
        for (script, mouse, keyb) in (
        &mut scripts,
        (&mut mouses).maybe(),
        (&mut keybs).maybe(),
        ).join() {
            // Best effort load of the script
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

            if !script.initialized {
                script.initialized = true;
                match self.engine.call_fn::<(), rhai::Map>(
                    &mut self.scope,
                    ast,
                    "spawn",
                    (),
                ) {
                    Ok(new_self) => script.object_self = new_self,
                    Err(e) => crate::log::error(&format!("spawn failed: {:?}", e)),
                }
            }

            if let Some(keyb_some) = keyb {
                for e in keyb_some.events.drain(..) {
                    match self.engine.call_fn::<(rhai::Map, String, bool), rhai::Map>(
                        &mut self.scope,
                        ast,
                        "on_kb",
                        (script.object_self.clone(), e.0.clone(), e.1),
                    ) {
                        Ok(new_self) => script.object_self = new_self,
                        Err(e) => crate::log::error(&format!("on_kb failed: {:?}", e)),
                    }
                }
            }

            if let Some(mouse_some) = mouse {
                if mouse_some.l_is_clicked {
                    mouse_some.l_is_clicked = false;
                    match self.engine.call_fn::<(rhai::Map,), rhai::Map>(
                        &mut self.scope,
                        ast,
                        "on_lclick",
                        (script.object_self.clone(),),
                    ) {
                        Ok(new_self) => script.object_self = new_self,
                        Err(e) => crate::log::error(&format!("lclick failed: {:?}", e)),
                    }
                    println!("{:?}", script.object_self);
                }
            }
        }
    }
}