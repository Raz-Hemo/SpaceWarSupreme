pub struct MeshInstance {
    world_matrix: cgmath::Matrix4<f32>,
}

pub struct StaticMeshSystem {
    instances: HashMap<Arc<Model>, Vec<MeshInstance>>
}

impl<'a> specs::System<'a> for StaticMeshSystem {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}