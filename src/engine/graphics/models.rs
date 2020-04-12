use std::collections::HashMap;
use glium::{Display, VertexBuffer, IndexBuffer};
use tobj;
use itertools::izip;

use super::vertex::Vertex;

pub struct Model<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CachedModel<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> Model<V> {
    fn from_cache<P: AsRef<std::path::Path>>(filename: P) -> crate::utils::SWSResult<CachedModel<V>> {
        match std::fs::File::open(filename) {
            Ok(file) => {
                let reader = std::io::BufReader::new(file);
                match bincode::deserialize_from(reader) {
                    Ok(data) => Ok(data),
                    Err(e) => Err(format!("Failed loading cached model: {:?}", e))
                }
            },
            Err(e) => Err(format!("Cached model does not exist: {:?}", e))
        }
    }

    fn from_obj<P: AsRef<std::path::Path>>(filename: P) -> crate::utils::SWSResult<CachedModel<Vertex>> {
        let obj = tobj::load_obj(filename.as_ref());
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

        Ok(CachedModel {vertices, indices})
    }

    pub fn from_data(cached: CachedModel<V>, display: &Display)
    -> crate::utils::SWSResult<Model<V>> {
        let vertices = match VertexBuffer::immutable(display, &cached.vertices) {
            Ok(buf) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };
        let indices = match IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &cached.indices) {
            Ok(buf) => buf,
            Err(e) => return Err(format!("{:?}", e)),
        };

        Ok(Model {
            vertices,
            indices,
        })
    }

    pub fn from<P: AsRef<std::path::Path>>(filename: P, display: &Display) -> crate::utils::SWSResult<Model<Vertex>> {
        let path = std::path::PathBuf::from("./resources/models/").join(&filename);
        let model_data = match crate::utils::should_load_from_cache(&path) {
            (true, Some(cache_path)) => Model::from_cache(cache_path),
            (false, Some(cache_path)) => {
                let m = Model::<Vertex>::from_obj(path);
                if let Err(e) = m {
                    crate::log::error(&format!("{:?}", e));
                    return Err(e)
                }
                let m = m.unwrap();
                if let Ok(mut file) = std::fs::File::create(&cache_path) {
                    match bincode::serialize_into(&mut file, &m) {
                        Ok(_) => (),
                        Err(e) => crate::log::error(&format!("Error caching {:?}: {:?}", filename.as_ref(), e)),
                    }
                }
                Ok(m)
            },
            _ => {
                Model::<Vertex>::from_obj(path)
            }
        };

        match model_data {
            Ok(data) => Model::from_data(data, display),
            Err(e) => Err(e),
        }
    }

    pub fn cube(display: &Display) -> Model<Vertex> {
        Model::<Vertex>::from_data(CachedModel {
            vertices: vec![
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
                }
            ],
            indices: vec![
                2,  1,  0,  3,  2,  0,  // front
                6,  5,  4,  7,  6,  4,  // right
                10,  9,  8, 11,  10, 8, // back
                14, 13, 12, 15, 14, 12, // left
                18, 17, 16, 19, 18, 16, // top
                22, 21, 20, 23, 22, 20  // bottom
            ]
        }, display).expect("Failed to create cube")
    }
}

pub struct ModelsManager {
    models: HashMap<String, Model<Vertex>>,
    default_model: Model<Vertex>,
}
impl ModelsManager {
    pub fn new(display: &Display) -> ModelsManager {
        ModelsManager {
            models: HashMap::new(),
            default_model: Model::<Vertex>::cube(display),
        }
    }

    pub fn get(&self, name: &str) -> &Model<Vertex> {
        self.models.get(name).unwrap_or(&self.default_model)
    }

    pub fn try_load(&mut self, display: &Display, name: &str) -> crate::utils::SWSResult<()> {
        if self.models.contains_key(name) {
            return Ok(())
        }

        match Model::<Vertex>::from(name, display) {
            Ok(m) => {self.models.insert(String::from(name), m); Ok(())},
            Err(e) => Err(e)
        }
    }
}