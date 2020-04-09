pub struct ScriptingComponent {
    pub path: String,
    pub object_self: rhai::Map,
    pub initialized: bool,
}

impl ScriptingComponent {
    pub fn new(path: &str) -> ScriptingComponent {
        ScriptingComponent {
            path: String::from(path),
            object_self: rhai::Map::new(),
            initialized: false,
        }
    }
}

impl specs::Component for ScriptingComponent {
    type Storage = specs::VecStorage<Self>;
}
