use crate::engine::prelude::*;
use std::collections::HashMap;
use glium::{Display, VertexBuffer, IndexBuffer};
use itertools::izip;

use super::vertex::Vertex;

pub struct Primitive<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> {
    pub vertices: VertexBuffer<V>,
    pub indices: IndexBuffer<u32>,
    pub albedo: Option<glium::texture::CompressedSrgbTexture2d>,
    pub normalmap: Option<glium::texture::CompressedTexture2d>,
    pub metal_roughness: Option<glium::texture::CompressedTexture2d>,
    pub occlusion: Option<glium::texture::CompressedTexture2d>,
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
    pub albedo: Option<Vec<u8>>,
    pub normalmap: Option<Vec<u8>>,
    pub metal_roughness: Option<Vec<u8>>,
    pub occlusion: Option<Vec<u8>>,
}

impl<V: serde::Serialize + serde::de::DeserializeOwned + glium::Vertex + Copy> Model<V> {
    fn from_cache<P: AsRef<std::path::Path>>(filename: P) -> anyhow::Result<CachedModel<V>> {
        use anyhow::Context;
        bincode::deserialize_from(std::io::BufReader::new(
            std::fs::File::open(filename).context("Cached model does not exist")?
        )).context("Failed loading cached model")
    }

    fn gltf_get_texture<P: AsRef<std::path::Path>>
    (tex: &gltf::Texture<'_>,
    buffers: &Vec<gltf::buffer::Data>,
    filename: P) -> anyhow::Result<Vec<u8>> {
        use anyhow::Context;
        Ok(match tex.source().source() {
            gltf::image::Source::View { view, mime_type } => {
                let parent_buffer_data = &buffers[view.buffer().index()].0;
                let begin = view.offset();
                let end = begin + view.length();
                let data = &parent_buffer_data[begin..end];
                match mime_type {
                    "image/jpeg" => data.iter().cloned().collect::<Vec<u8>>(),
                    "image/png" => data.iter().cloned().collect::<Vec<u8>>(),
                    _ => Err(anyhow!("Unsupported image type (image: {:?}, mime_type: {})", tex.name(), mime_type))?,
                }
            },
            gltf::image::Source::Uri { uri, mime_type } => {
                if uri.starts_with("data:") {
                    let encoded = uri.split(',').nth(1).context("Failed to process uri")?;
                    let data = base64::decode(&encoded).context("Failed to decode base64")?;
                    let mime_type = if let Some(ty) = mime_type {
                        ty
                    } else {
                        uri.split(',')
                            .nth(0).context("Failed to process uri")?
                            .split(':')
                            .nth(1).context("Failed to process uri")?
                            .split(';')
                            .nth(0).context("Failed to process uri")?
                    };

                    match mime_type {
                        "image/jpeg" => data,
                        "image/png" => data,
                        _ => Err(anyhow!("Unsupported image type (image: {:?}, mime_type: {})", tex.name(), mime_type))?,
                    }
                }
                else if let Some(mime_type) = mime_type {
                    let path = filename.as_ref().parent().unwrap_or(std::path::Path::new("./")).join(uri);
                    match mime_type {
                        "image/jpeg" => std::fs::read(path)?,
                        "image/png" => std::fs::read(path)?,
                        _ => Err(anyhow!("Unsupported image type (image: {:?}, mime_type: {})", tex.name(), mime_type))?,
                    }
                }
                else {
                    std::fs::read(filename.as_ref().parent().unwrap_or(std::path::Path::new("./")).join(uri))?
                }
            },
        })
    }

    fn from_gltf<P: AsRef<std::path::Path>>(filename: P) -> anyhow::Result<CachedModel<Vertex>> {
        let (gltf, buffers, _images) = gltf::import(filename.as_ref())?;
        let mut result = CachedModel::<Vertex>::default();

        for mesh in gltf.meshes() {
            for p in mesh.primitives() {
                let mut vertices: Vec<Vertex> = Vec::new();
                let mut albedo: Option<Vec<u8>> = None;
                if let Some(color) = p.material().pbr_metallic_roughness().base_color_texture() {
                    albedo = Some(Self::gltf_get_texture(&color.texture(), &buffers, filename.as_ref())?);
                }

                let mut normalmap: Option<Vec<u8>> = None;
                if let Some(norms) = p.material().normal_texture() {
                    normalmap = Some(Self::gltf_get_texture(&norms.texture(), &buffers, filename.as_ref())?);
                }

                let mut metal_roughness: Option<Vec<u8>> = None;
                if let Some(mr) = p.material().pbr_metallic_roughness().metallic_roughness_texture() {
                    metal_roughness = Some(Self::gltf_get_texture(&mr.texture(), &buffers, filename.as_ref())?);
                }

                let mut occlusion: Option<Vec<u8>> = None;
                if let Some(occ) = p.material().pbr_metallic_roughness().metallic_roughness_texture() {
                    occlusion = Some(Self::gltf_get_texture(&occ.texture(), &buffers, filename.as_ref())?);
                }

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
                result.primitives.push(CachedPrimitive {vertices, indices, albedo, normalmap, metal_roughness, occlusion})
            }
        }

        Ok(result)
    }

    pub fn from_data(cached: CachedModel<V>, display: &Display)
    -> anyhow::Result<Model<V>> {
        use glium::texture::{RawImage2d, CompressedSrgbTexture2d, CompressedTexture2d};
        use anyhow::Context;
        let mut primitives = Vec::new();

        for p in cached.primitives {
            primitives.push(Primitive {
                vertices: VertexBuffer::immutable(display, &p.vertices)?,
                indices: IndexBuffer::immutable(display, glium::index::PrimitiveType::TrianglesList, &p.indices)?,
                albedo: if let Some(alb) = p.albedo {
                    let tex = image::load_from_memory(&alb).context("Failed to load albedo texture")?.to_rgba();
                    let dims =  tex.dimensions();
                    let raw_image = RawImage2d::from_raw_rgba(tex.into_raw(), dims);
                    Some(CompressedSrgbTexture2d::new(display, raw_image).context("Failed to create albedo texture")?)
                } else { None },
                normalmap: if let Some(norm) = p.normalmap {
                    let tex = image::load_from_memory(&norm).context("Failed to load normal map")?.to_rgb();
                    let dims = tex.dimensions();
                    let raw_image = RawImage2d::from_raw_rgb(tex.into_raw(), dims);
                    Some(CompressedTexture2d::new(display, raw_image).context("Failed to create normalmap")?)
                } else { None },
                metal_roughness: if let Some(mr) = p.metal_roughness {
                    let tex = image::load_from_memory(&mr).context("Failed to load metal/roughness map")?.to_rgb();
                    let dims = tex.dimensions();
                    let raw_image = RawImage2d::from_raw_rgb(tex.into_raw(), dims);
                    Some(CompressedTexture2d::new(display, raw_image).context("Failed to create metal/roughness map")?)
                } else { None },
                occlusion: if let Some(occ) = p.occlusion {
                    let tex = image::load_from_memory(&occ).context("Failed to load AO map")?.to_rgb();
                    let dims = tex.dimensions();
                    let raw_image = RawImage2d::from_raw_rgb(tex.into_raw(), dims);
                    Some(CompressedTexture2d::new(display, raw_image).context("Failed to create AO map")?)
                } else { None },
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
            ],
            albedo: None,
            normalmap: None,
            metal_roughness: None,
            occlusion: None,
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