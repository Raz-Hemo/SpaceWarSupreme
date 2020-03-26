use std::sync::Arc;
use std::collections::HashMap;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::Queue;

#[derive(Default, Debug, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texcoord: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, normal, texcoord);

pub struct Model {
    pub vertices: Arc<ImmutableBuffer<[Vertex]>>,
    pub indices: Arc<ImmutableBuffer<[u32]>>,
}

impl Model {
    pub fn from<P: AsRef<std::path::Path>>(filename: P, queue: &Arc<Queue>) -> Option<Arc<Model>> {
        Some(Model::cube(queue))
        //Some(Model {
        //    vertices: ImmutableBuffer::from_iter(, BufferUsage::vertex_buffer(), queue.clone()),
        //    indices:,
        //})
    }

    pub fn cube(queue: &Arc<Queue>) -> Arc<Model> {
        Arc::new(Model {
            vertices: ImmutableBuffer::from_iter([
                // +z
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                },


                // -z
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                },


                // +x
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },


                // -x
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                },


                // +y
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },

                // -y
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
            ].iter().cloned(), BufferUsage::vertex_buffer(), queue.clone()).expect("Failed to create cube vertex buffer").0,
            indices: ImmutableBuffer::from_iter([
                2,  1,  0,  3,  2,  0,  // front
                6,  5,  4,  7,  6,  4,  // right
                10,  9,  8, 11,  10, 8, // back
                14, 13, 12, 15, 14, 12, // left
                18, 17, 16, 19, 18, 16, // top
                22, 21, 20, 23, 22, 20  // bottom
            ].iter().cloned(), BufferUsage::index_buffer(), queue.clone()).expect("Failed to create cube index buffer").0
        })
    }
}

pub type ModelID = u64;
pub struct ModelsManager {
    // The running counter
    current_model_id: ModelID,
    model_ids: HashMap<String, ModelID>,

    queue: Arc<Queue>,
    models: HashMap<ModelID, Arc<Model>>,
}

impl ModelsManager {
    pub fn new(queue: &Arc<Queue>) -> ModelsManager {
        let mut result = ModelsManager {
            current_model_id: 0,
            queue: queue.clone(),
            model_ids: HashMap::new(),
            models: HashMap::new(),
        };

        result.add_model("cube", Model::cube(queue));

        result
    }

    // Insert a model into the manager and return its ID
    pub fn add_model(&mut self, name: &str, m: Arc<Model>) -> ModelID {
        self.model_ids.insert(String::from(name), self.current_model_id);
        self.models.insert(self.current_model_id, m);
        self.current_model_id += 1;

        self.current_model_id - 1
    }

    pub fn get_id(&mut self, name: &str) -> ModelID {
        // If loaded, return it
        match self.model_ids.get(name) {
            Some(id) => return *id,
            None => ()
        }

        // Try load
        let model = match Model::from(String::from("models\\") + name, &self.queue) {
            Some(data) => data,
            None => {
                crate::log::warning(&format!("Model {} not found, loading cube instead", name));
                return 0
            }
        };

        self.add_model(name, model.clone())
    }

    pub fn id_to_model(&self, id: &ModelID) -> Option<&Arc<Model>> {
        self.models.get(id)
    }
}