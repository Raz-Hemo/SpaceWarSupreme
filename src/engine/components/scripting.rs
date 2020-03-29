#[derive(Debug)]
pub struct ScriptingComponent {
    script: String,
}

impl specs::Component for ScriptingComponent {
    type Storage = specs::HashMapStorage<Self>;
}
