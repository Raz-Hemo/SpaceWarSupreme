use std::collections::HashMap;
use glium::{Display, VertexBuffer, IndexBuffer};
use tobj;
use itertools::izip;

use super::vertex::Vertex;

pub struct Model {
    pub vertices: VertexBuffer<Vertex>,
    pub indices: IndexBuffer<u32>,
}

impl Model {
    pub fn from<P: AsRef<std::path::Path>>(filename: P, display: &Display) -> crate::utils::SWSResult<Model> {
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
                    position: [p[0], p[1], p[2]],
                    normal: [n[0], n[1], n[2]],
                    texcoord: [t[0], t[1]],
                });
            }
        }

        let vertices = match VertexBuffer::immutable(display, &vertices) {
            Ok(buf) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let indices = match IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &indices) {
            Ok(buf) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };
        Ok(Model {
            vertices,
            indices,
        })
    }

    pub fn cube(display: &Display) -> Model {
        Model {
            vertices: VertexBuffer::immutable(
                display,
                // +z
                &[Vertex {
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
                }]).expect("Failed to create cube vertex buffer"),
            indices: IndexBuffer::immutable(
                display,
                glium::index::PrimitiveType::TrianglesList, 
                &[
                    2,  1,  0,  3,  2,  0,  // front
                    6,  5,  4,  7,  6,  4,  // right
                    10,  9,  8, 11,  10, 8, // back
                    14, 13, 12, 15, 14, 12, // left
                    18, 17, 16, 19, 18, 16, // top
                    22, 21, 20, 23, 22, 20  // bottom
            ]).expect("Failed to create cube index buffer")
        }
    }
}

pub struct ModelsManager {
    models: HashMap<String, Model>,
    default_model: Model,
}
impl ModelsManager {
    pub fn new(display: &Display) -> ModelsManager {
        ModelsManager {
            models: HashMap::new(),
            default_model: Model::cube(display),
        }
    }

    pub fn get(&self, name: &str) -> &Model {
        self.models.get(name).unwrap_or(&self.default_model)
    }

    pub fn try_load(&mut self, display: &Display, name: &str) -> crate::utils::SWSResult<()> {
        if self.models.contains_key(name) {
            return Ok(())
        }

        match Model::from(name, display) {
            Ok(m) => {self.models.insert(String::from(name), m); Ok(())},
            Err(e) => Err(format!("{:?}", e))
        }
    }
}