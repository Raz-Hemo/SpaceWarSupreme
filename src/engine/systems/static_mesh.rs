use std::collections::HashMap;
use cgmath::SquareMatrix;
use specs::{ReadStorage, Entity, Entities};
use crate::engine::components::{StaticMeshComponent, MouseComponent, TransformComponent};

#[derive(Clone, Copy)]
pub struct MeshInstance {
    pub world_matrix: [[f32; 4]; 4],
    pub entity: u32,
}
glium::implement_vertex!(MeshInstance, world_matrix, entity);

impl Default for MeshInstance {
    fn default() -> Self {
        MeshInstance {
            // An identity matrix
            world_matrix: cgmath::Matrix4::identity().into(),
            entity: 65535,
        }
    }
}
impl MeshInstance {
    pub fn new() -> MeshInstance {
        MeshInstance::default()
    }
}

pub struct StaticMeshSystem {
    // Keeps track of the pickable entities and the worlds they belong to
    pickables: Vec<(u32, Entity)>,
    next_frame_instances: HashMap<String, Vec<MeshInstance>>,
    cur_world_index: u32, // Keeps track of the worlds rendered in a single frame
}

impl StaticMeshSystem {
    pub fn new() -> StaticMeshSystem {
        StaticMeshSystem {
            pickables: Vec::new(),
            next_frame_instances: HashMap::new(),
            cur_world_index: 0,
        }
    }

    /// Should be called between frames, but not during - to preserve the pickable ordering.
    pub fn get_instances_and_flush(&mut self)
    -> (HashMap<String, Vec<MeshInstance>>, Vec<(u32, Entity)>) {
        let desired_capacity = self.pickables.len();
        let result_pickables = std::mem::replace(
            &mut self.pickables,
            Vec::with_capacity(desired_capacity)
        );

        let desired_capacity = self.next_frame_instances.capacity();
        let result_instances = std::mem::replace(
            &mut self.next_frame_instances,
            HashMap::with_capacity(desired_capacity)
        );

        self.next_frame_instances.clear();
        self.pickables.clear();
        self.cur_world_index = 0;

        (result_instances, result_pickables)
    }
}

impl<'a> specs::System<'a> for StaticMeshSystem {
    type SystemData = (
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, StaticMeshComponent>,
        ReadStorage<'a, MouseComponent>,
        Entities<'a>
    );

    fn run(&mut self, (transforms, meshes, mouses, ents): Self::SystemData) {
        use specs::Join;

        for (trans, mesh, mouse, ent) in 
                (&transforms, &meshes, (&mouses).maybe(), &ents).join() {
            self.next_frame_instances.entry(mesh.model.clone()).or_default().push(MeshInstance {
                world_matrix: (trans.transform * mesh.rel_transform).into(),
                entity: if mouse.is_none() {0} else {self.pickables.len() as u32 + 1},
            });
            self.pickables.push((self.cur_world_index, ent));
        }
        self.cur_world_index += 1;
    }
}