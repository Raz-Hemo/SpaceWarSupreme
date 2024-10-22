use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 3],
    pub texcoord: [f32; 2],
}
glium::implement_vertex!(Vertex, position, normal, tangent, texcoord);

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub texcoord: [f32; 2],
}
glium::implement_vertex!(Vertex2d, position, texcoord);

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VertexSkybox {
    pub position: [f32; 3],
}
glium::implement_vertex!(VertexSkybox, position);