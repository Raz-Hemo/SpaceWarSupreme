use std::collections::HashMap;
use std::sync::Arc;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::Queue;
use cgmath::SquareMatrix;
use specs::{ReadStorage, Entity, Entities};
use crate::engine::graphics::ModelID;
use crate::engine::components::{StaticMeshComponent, MouseComponent, TransformComponent};

pub type InstanceBuffers = Vec<(ModelID, Arc<ImmutableBuffer<[MeshInstance]>>)>;
pub type Pickables = Vec<(u32, Entity)>;

#[derive(Clone, Copy)]
pub struct MeshInstance {
    pub world_matrix: [[f32; 4]; 4],
    pub entity: u32,
}

vulkano::impl_vertex!(MeshInstance, world_matrix, entity);
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
    pickables: Pickables,
    next_frame_instances: HashMap<ModelID, Vec<MeshInstance>>,
    cur_world_index: u32, // Keeps track of the worlds rendered in a single frame
    queue: Arc<Queue>,
}

impl StaticMeshSystem {
    pub fn new(queue: Arc<Queue>) -> StaticMeshSystem {
        StaticMeshSystem {
            pickables: Vec::new(),
            next_frame_instances: HashMap::new(),
            cur_world_index: 0,
            queue,
        }
    }

    /// Should be called between frames, but not during - to preserve the pickable ordering.
    pub fn get_instances_and_flush(&mut self) -> (InstanceBuffers, Vec<(u32, Entity)>) {
        let mut result_instances = Vec::new();
        for (k, v) in self.next_frame_instances.iter() {
            result_instances.push((
                k.clone(),
                ImmutableBuffer::from_iter(
                    v.iter().cloned(),
                    BufferUsage::vertex_buffer(),
                    self.queue.clone()
                ).expect("Failed to create instance buffer").0
            ));
        }
        

        let desired_capacity = self.pickables.len();
        let result_pickables = std::mem::replace(
            &mut self.pickables,
            Vec::with_capacity(desired_capacity)
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
                entity: if mouse.is_none() {std::u32::MAX} else {self.pickables.len() as u32},
            });
            self.pickables.push((self.cur_world_index, ent));
        }
        self.cur_world_index += 1;
    }
}