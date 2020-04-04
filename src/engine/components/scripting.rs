pub struct ScriptingComponent {
    pub path: String,
    pub scope: rhai::Scope<'static>,
}

impl ScriptingComponent {
    pub fn new(path: &str) -> ScriptingComponent {
        ScriptingComponent {
            path: String::from(path),
            scope: rhai::Scope::new(),
        }
    }
}

impl specs::Component for ScriptingComponent {
    type Storage = specs::VecStorage<Self>;
}
