/// Describes a camera that the renderer can use.
pub trait Camera {
    /// Get a regular view matrix (rotation+translation)
    fn get_view(&self) -> nalgebra::Matrix4<f32>;

    /// Get view matrix without the translation component (for skyboxing)
    fn get_view_at_origin(&self) -> nalgebra::Matrix4<f32>;
}
