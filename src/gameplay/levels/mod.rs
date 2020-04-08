/// A level manages one or more spaces, which are independent ECS containers, and holds all
/// gameplay logic between them.
pub trait Level {
    fn iter_render(&mut self) -> SpaceIterator;
    fn iter_tickable(&mut self) -> SpaceIterator;
    fn iter_all(&mut self) -> SpaceIterator;
}

/// Allows external callers to iterate spaces of a level.
pub type SpaceIterator<'a> = Box<dyn Iterator<Item=&'a mut specs::World> + 'a>;

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
    world.insert(crate::engine::camera::Camera::new());

    world
}

pub mod spacewar;
