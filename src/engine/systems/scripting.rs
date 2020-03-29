use specs::ReadStorage;
use crate::engine::components::{ScriptingComponent, MouseComponent};

pub enum GameEvent {
    ChangeResolution(u32, u32),
}

pub struct ScriptingSystem {
    pub events: Vec<GameEvent>,
}

impl ScriptingSystem {
    pub fn new() -> ScriptingSystem {
        ScriptingSystem {
            events: Vec::new(),
        }
    }
}

impl<'a> specs::System<'a> for ScriptingSystem {
    type SystemData = (
        ReadStorage<'a, ScriptingComponent>,
        ReadStorage<'a, MouseComponent>,
    );

    fn run(&mut self, (scripts, mouses): Self::SystemData) {
        use specs::Join;

        for (script, mouse) in (&scripts, &mouses).join() {

        }
    }
}