use std::collections::HashMap;
use std::sync::Arc;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::Queue;
use cgmath::SquareMatrix;
use specs::{ReadStorage, Entity, Entities};
use crate::engine::graphics::ModelID;
use crate::engine::components::{StaticMeshComponent, MouseComponent, TransformComponent};

pub type InstanceBuffers = Vec<(ModelID, Arc<ImmutableBuffer<[MeshInstance]>>)>;

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
    pub next_instance_buffers: InstanceBuffers,
    next_frame_instances: HashMap<ModelID, Vec<MeshInstance>>,
    queue: Arc<Queue>,
    pub pickables: Vec<Entity>,
}

impl StaticMeshSystem {
    pub fn new(queue: Arc<Queue>) -> StaticMeshSystem {
        StaticMeshSystem {
            next_frame_instances: HashMap::new(),
            next_instance_buffers: Vec::new(),
            queue,
            pickables: Vec::new(),
        }
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

        self.next_frame_instances.clear();
        self.next_instance_buffers.clear();
        self.pickables.clear();

        for (trans, mesh, mouse, ent) in 
                (&transforms, &meshes, (&mouses).maybe(), &ents).join() {
            self.next_frame_instances.entry(mesh.model).or_default().push(MeshInstance {
                world_matrix: (trans.transform * mesh.rel_transform).into(),
                entity: if mouse.is_none() {std::u32::MAX} else {self.pickables.len() as u32},
            });
            self.pickables.push(ent);
        }
        for (k, v) in self.next_frame_instances.iter() {
            self.next_instance_buffers.push((*k, ImmutableBuffer::from_iter(
                v.iter().cloned(),
                BufferUsage::vertex_buffer(),
                self.queue.clone()
            ).expect("Failed to create instance buffer").0
            ));
        }
    }
}