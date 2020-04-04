use specs::WriteStorage;
use crate::engine::components::ScriptingComponent;


pub struct ScriptingPreloadSystem {
    pub used_scripts: Vec<String>
}
impl ScriptingPreloadSystem {
    pub fn new() -> ScriptingPreloadSystem {
        ScriptingPreloadSystem {
            used_scripts: Vec::new()
        }
    }
}

impl<'a> specs::System<'a> for ScriptingPreloadSystem {
    type SystemData = WriteStorage<'a, ScriptingComponent>;

    fn run(&mut self, scripts: Self::SystemData) {
        use specs::Join;
        let mut result = Vec::new();

        // First call the mouse functions (on_click, etc.)
        for script in scripts.join() {
            result.push(script.path.clone());
        }
    }
}