pub struct ScriptingComponent {
    pub path: String,
}

impl ScriptingComponent {
    pub fn new(path: &str) -> ScriptingComponent {
        ScriptingComponent {
            path: String::from(path),
        }
    }
}

impl specs::Component for ScriptingComponent {
    type Storage = specs::VecStorage<Self>;
}
