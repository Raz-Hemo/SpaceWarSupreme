use crate::engine::prelude::*;
use std::collections::HashMap;
use glium::{Display, VertexBuffer, IndexBuffer};
use itertools::izip;

use super::vertex::Vertex;

pub struct Primitive<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>,
}

pub struct Model<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> {
    pub primitives: Vec<Primitive<V>>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct CachedModel<V> {
    pub primitives: Vec<CachedPrimitive<V>>,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct CachedPrimitive<V> {
    pub vertices: Vec<V>,
    pub indices: Vec<u32>,
}

impl<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> Model<V> {
    fn from_cache<P: AsRef<std::path::Path>>(filename: P) -> anyhow::Result<CachedModel<V>> {
        use anyhow::Context;
        bincode::deserialize_from(std::io::BufReader::new(
            std::fs::File::open(filename).context("Cached model does not exist")?
        )).context("Failed loading cached model")
    }

    fn from_gltf<P: AsRef<std::path::Path>>(filename: P) -> anyhow::Result<CachedModel<Vertex>> {
        let (gltf, buffers, _images) = gltf::import(filename)?;
        let mut result = CachedModel::<Vertex>::default();

        for mesh in gltf.meshes() {
            for p in mesh.primitives() {
                let mut vertices: Vec<Vertex> = Vec::new();

                let reader = p.reader(|buffer| Some(&buffers[buffer.index()]));
                let indices = reader.read_indices().ok_or(
                    anyhow!("Mesh has no indices"))?.into_u32().collect();

                let positions = reader.read_positions().ok_or(
                    anyhow!("Mesh has no positions"))?;
                let texcoords = reader.read_tex_coords(0).ok_or(
                    anyhow!("Mesh has no texcoords"))?.into_f32();
                let normals = reader.read_normals().ok_or(
                    anyhow!("Mesh has no normals"))?;
                let tangents = reader.read_tangents().ok_or(
                    anyhow!("Mesh has no tangents"))?;

                for (p, tx, n, t) in izip!(positions, texcoords, normals, tangents) {
                    use std::convert::TryInto;
                    vertices.push(Vertex {
                        position: p,
                        texcoord: tx,
                        normal: n,
                        tangent: t[..3].try_into()?,
                    });
                }
                result.primitives.push(CachedPrimitive {vertices, indices})
            }
        }

        Ok(result)
    }

    pub fn from_data(cached: CachedModel<V>, display: &Display)
    -> anyhow::Result<Model<V>> {
        let mut primitives = Vec::new();
        for p in cached.primitives {
            primitives.push(Primitive {
                vertices: VertexBuffer::immutable(display, &p.vertices)?,
                indices: IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &p.indices)?,
            });
        }
        Ok(Model {primitives})
    }

    pub fn from<P: AsRef<std::path::Path>>(filename: P, display: &Display) -> anyhow::Result<Model<Vertex>> {
        use anyhow::Context;
        let path = std::path::PathBuf::from("./resources/models/").join(&filename);
        let model_data = match utils::should_load_from_cache(&path) {
            (true, Some(cache_path)) => Model::from_cache(cache_path),
            (false, Some(cache_path)) => {
                let m = Model::<Vertex>::from_gltf(path)?;
                let mut file = std::fs::File::create(&cache_path).context("Error creating cache file")?;
                bincode::serialize_into(&mut file, &m).context("Error serializing")?;
                Ok(m)
            },
            _ => {
                Model::<Vertex>::from_gltf(path)
            }
        };

        match model_data {
            Ok(data) => Model::from_data(data, display),
            Err(e) => Err(e),
        }
    }

    pub fn cube(display: &Display) -> Model<Vertex> {
        Model::<Vertex>::from_data(CachedModel { primitives: vec![CachedPrimitive {
            vertices: vec![
                // +z
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [0.0, 0.0, 1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },


                // -z
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [0.0, 0.0, -1.0],
                    texcoord: [0.0, 0.0],
                    tangent: [1.0, 0.0, 0.0],
                },


                // +x
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },


                // -x
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [-1.0, 0.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 1.0, 0.0],
                },


                // +y
                Vertex {
                    position: [-1.0, 1.0, 1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0, 1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0, -1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [-1.0, 1.0, -1.0],
                    normal: [0.0, 1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },

                // -y
                Vertex {
                    position: [-1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0, 1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                },
                Vertex {
                    position: [-1.0, -1.0, -1.0],
                    normal: [0.0, -1.0, 0.0],
                    texcoord: [0.0, 0.0],
                    tangent: [0.0, 0.0, 1.0],
                }
            ],
            indices: vec![
                0,  1,  2,  0,  2,  3,  // front
                4,  5,  6,  4,  6,  7,  // right
                8,  9,  10,  8, 10, 11,  // back
                12, 13, 14, 12, 14, 15, // left
                16, 17, 18, 16, 18, 19, // top
                22, 21, 20, 23, 22, 20  // bottom
            ]
        }]}, display).expect("Failed to create cube")
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

    pub fn try_load(&mut self, display: &Display, name: &str) -> anyhow::Result<()> {
        use anyhow::Context;
        if self.models.contains_key(name) {
            return Ok(())
        }

        self.models.insert(
            String::from(name),
            Model::<Vertex>::from(name, display).context(format!("Failed to load {}", name))?
        );
        Ok(())
    }
}