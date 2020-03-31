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

#[derive(Clone)]
pub struct ModelID {
    pub name: String,
    pub cached: Option<Arc<Model>>,
}
impl std::hash::Hash for ModelID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}
impl PartialEq for ModelID {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for ModelID {}
impl ModelID {
    pub fn from(name: &str) -> ModelID {
        ModelID {
            name: String::from(name),
            cached: None,
        }
    }
}

pub struct ModelsManager {
    queue: Arc<Queue>,
    models: HashMap<String, Arc<Model>>,
    default_model: Arc<Model>,
}
impl ModelsManager {
    pub fn new(queue: &Arc<Queue>) -> ModelsManager {
        ModelsManager {
            queue: queue.clone(),
            models: HashMap::new(),
            default_model: Model::cube(queue),
        }
    }

    pub fn get(&mut self, name: &str) -> Arc<Model> {
        // If loaded, return it
        match self.models.get(name) {
            Some(m) => return m.clone(),
            None => ()
        }

        // Try load
        let model = match Model::from(name, &self.queue) {
            Ok(data) => data,
            Err(e) => {
                crate::log::warning(&format!(
                    "Model {} not found ({}), loading cube instead", name, e
                ));
                return self.default_model.clone()
            }
        };

        self.models.insert(String::from(name), model.clone());
        model
    }
}