#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub texcoord: [f32; 2],
}
glium::implement_vertex!(Vertex, position, normal, texcoord);

#[derive(Default, Debug, Clone, Copy)]
pub struct Vertex2d {
    pub position: [f32; 2],
    pub texcoord: [f32; 2],
}
glium::implement_vertex!(Vertex2d, position, texcoord);
