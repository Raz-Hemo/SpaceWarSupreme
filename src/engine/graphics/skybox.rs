use glium::Display;
use super::vertex::VertexSkybox;
use super::models::{Model, CachedModel, CachedPrimitive};

impl super::Renderer {
    pub fn create_skybox_vbuffer(display: &Display) -> Model<VertexSkybox> {
        Model::from_data(
        CachedModel { primitives: vec![CachedPrimitive {
            vertices: vec![
                // +z
                VertexSkybox {
                    position: [1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [-1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [-1.0, -1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, 1.0],
                },

                // -z
                VertexSkybox {
                    position: [1.0, 1.0, -1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, -1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, 1.0, -1.0],
                },


                // +x
                VertexSkybox {
                    position: [1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, -1.0],
                },
                VertexSkybox {
                    position: [1.0, 1.0, -1.0],
                },


                // -x
                VertexSkybox {
                    position: [-1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [-1.0, 1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, -1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, -1.0, 1.0],
                },


                // +y
                VertexSkybox {
                    position: [-1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, 1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, 1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, 1.0, -1.0],
                },

                // -y
                VertexSkybox {
                    position: [-1.0, -1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, 1.0],
                },
                VertexSkybox {
                    position: [1.0, -1.0, -1.0],
                },
                VertexSkybox {
                    position: [-1.0, -1.0, -1.0],
                }
            ],
            indices: vec![
                2,  1,  0,  3,  2,  0,  // front
                6,  5,  4,  7,  6,  4,  // right
                10, 9,  8,  11, 10, 8,  // back
                14, 13, 12, 15, 14, 12, // left
                18, 17, 16, 19, 18, 16, // top
                20, 21, 22, 20, 22, 23  // bottom
            ]
        }]},
        display).expect("Failed to create skybox vbuffer")
    }
}
