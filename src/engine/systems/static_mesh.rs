use std::collections::HashMap;
use std::sync::Arc;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::{Device, Queue};
use cgmath::SquareMatrix;
use specs::ReadStorage;
use crate::engine::graphics::ModelID;
use crate::engine::components::StaticMeshComponent;

pub type InstanceBuffers = Vec<(ModelID, Arc<ImmutableBuffer<[MeshInstance]>>)>;

#[derive(Clone, Copy)]
pub struct MeshInstance {
    pub world_matrix: [[f32; 4]; 4],
}

vulkano::impl_vertex!(MeshInstance, world_matrix);
impl Default for MeshInstance {
    fn default() -> Self {
        MeshInstance {
            // An identity matrix
            world_matrix: cgmath::Matrix4::identity().into()
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
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl StaticMeshSystem {
    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> StaticMeshSystem {
        StaticMeshSystem {
            next_frame_instances: HashMap::new(),
            next_instance_buffers: Vec::new(),
            device,
            queue
        }
    }
}

impl<'a> specs::System<'a> for StaticMeshSystem {
    type SystemData = ReadStorage<'a, StaticMeshComponent>;

    fn run(&mut self, components: Self::SystemData) {
        use specs::Join;

        self.next_frame_instances.clear();
        self.next_instance_buffers.clear();

        for c in components.join() {
            self.next_frame_instances.entry(c.model).or_default().push(c.mesh_instance);
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