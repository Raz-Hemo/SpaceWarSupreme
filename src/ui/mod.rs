use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};


#[derive(Default, Debug, Clone)]
struct UIVertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(UIVertex, position);

pub trait Button {
    fn hover();
    fn click();
}
