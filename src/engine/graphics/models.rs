use std::sync::Arc;
use std::collections::HashMap;
use vulkano::buffer::{ImmutableBuffer, BufferUsage};
use vulkano::device::Queue;
use tobj;
use itertools::izip;

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
    pub fn from<P: AsRef<std::path::Path>>(filename: P, queue: &Arc<Queue>) -> crate::utils::SWSResult<Arc<Model>> {
        let obj = tobj::load_obj(&std::path::Path::new("./resources/models/").join(filename));
        if let Err(e) = obj {
            return Err(format!("Model does not exist: {:?}", e));
        }
        let (models, _materials) = obj.unwrap();
        let mut indices: Vec<u32> = Vec::new();
        let mut vertices: Vec<Vertex> = Vec::new();

        for m in models.iter() {
            if m.mesh.texcoords.is_empty() {
                return Err(String::from("Model is missing texcoords"));
            }
            if m.mesh.normals.is_empty() {
                return Err(String::from("Model is missing normals"));
            }

            // Indices are model-relative, and we flatten them to a single buffer,
            // so add the base index where we put our model.
            let indices_base = vertices.len() as u32;
            for i in m.mesh.indices.iter() {
                indices.push(indices_base + i);
            }

            for (p, n, t) in izip!(m.mesh.positions.chunks(3), m.mesh.normals.chunks(3), m.mesh.texcoords.chunks(2))  {
                vertices.push(Vertex {
                    // Reverse Y axis because we are in Vulkan
                    position: [p[0], -p[1], p[2]],
                    normal: [n[0], n[1], n[2]],
                    texcoord: [t[0], t[1]],
                });
            }
        }

        let vertices = match ImmutableBuffer::from_iter(vertices.iter().cloned(), BufferUsage::vertex_buffer(), queue.clone()) {
            Ok((buf, _future)) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let indices = match ImmutableBuffer::from_iter(indices.iter().cloned(), BufferUsage::index_buffer(), queue.clone()) {
            Ok((buf, _future)) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };
        Ok(Arc::new(Model {
            vertices,
            indices,
        }))
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
        let model = match Model::from(name, &self.queue) {
            Ok(data) => data,
            Err(e) => {
                crate::log::warning(&format!(
                    "Model {} not found ({}), loading cube instead", name, e
                ));
                return 0
            }
        };

        self.add_model(name, model.clone())
    }

    pub fn id_to_model(&self, id: &ModelID) -> Option<&Arc<Model>> {
        self.models.get(id)
    }
}