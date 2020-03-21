use std::sync::Arc;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::Queue;

#[derive(Default, Debug, Clone)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    texcoord: [f32; 2],
}
vulkano::impl_vertex!(Vertex, position, normal, texcoord);

pub struct Model {
    vertices: Arc<ImmutableBuffer<[Vertex]>>,
    indices: Arc<ImmutableBuffer<[u32]>>,
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
                    position: [-1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                },

            ].iter().cloned(), BufferUsage::vertex_buffer(), queue.clone()).expect("Failed to create cube vertex buffer").0,
            indices: ImmutableBuffer::from_iter([
                0,  1,  2,  0,  2,  3,  // front
                4,  5,  6,  4,  6,  7,  // right
                8,  9,  10, 8,  10, 11, // back
                12, 13, 14, 12, 14, 15, // left
                16, 17, 18, 16, 18, 19, // top
                20, 21, 22, 20, 22, 23  // bottom
            ].iter().cloned(), BufferUsage::index_buffer(), queue.clone()).expect("Failed to create cube index buffer").0
        })
    }
}

pub struct ModelsManager {
    queue: Arc<Queue>,
    models: std::collections::HashMap<String, Arc<Model>>,
    default_model: Arc<Model>,
}

impl ModelsManager {
    pub fn new(queue: &Arc<Queue>) -> ModelsManager {
        ModelsManager {
            queue: queue.clone(),
            models: std::collections::HashMap::new(),
            default_model: Model::cube(queue),
        }
    }

    pub fn get(&mut self, name: &str) -> Arc<Model> {
        // If loaded, return it
        match self.models.get(name) {
            Some(model) => {return model.clone()},
            None => ()
        }

        // Try load
        let model = match Model::from(String::from("models\\") + name, &self.queue) {
            Some(data) => data,
            None => {
                crate::log::warning(&format!("Model {} not found, loading cube instead", name));
                self.default_model.clone()
            }
        };
        self.models.insert(String::from(name), model.clone());
        model
    }
}