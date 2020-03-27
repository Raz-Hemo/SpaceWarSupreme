/// A level is just a bootstrapper that creates the world (entities and components)
/// For example, a "main menu level" would initialize the buttons and decorations
/// for the main menu and their associated logic.
pub trait Level {
    fn load_level(&mut self, engine: &mut crate::engine::Engine);
}

pub mod spacewar;
