/// A level manages one or more spaces, which are independent ECS containers, and holds all
/// gameplay logic between them.
pub trait Level {
    /// Allows external callers to iterate spaces of a level.
    fn iter_spaces(&mut self) -> SpaceIterator;

    /// Tells the engine what space to send keyboard input to.
    fn get_active_space(&mut self) -> &mut specs::World;

    /// Set the active space. see `get_active_space` for more.
    fn set_active_space(&mut self, space: &str);
}
type SpaceIterator<'a> = Box<dyn Iterator<Item=&'a mut specs::World> + 'a>;

/// All the boilerplate of initializing a space
fn create_space() -> specs::World {
    use crate::engine::components;
    use specs::{WorldExt};

    let mut world = specs::World::new();
    world.register::<components::TransformComponent>();
    world.register::<components::StaticMeshComponent>();
    world.register::<components::MouseComponent>();
    world.register::<components::KeyboardComponent>();
    world.register::<components::ScriptingComponent>();
    world.register::<components::StaticSkyboxComponent>();
    world.insert(crate::engine::systems::KeyboardState {ctrl: false, shift: false, alt: false});

    world
}

pub mod spacewar;
