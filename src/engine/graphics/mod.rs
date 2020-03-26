pub mod window;

pub mod models;
pub use models::{ModelID, ModelsManager};

pub mod renderer;
pub use renderer::{Renderer, DrawResult};

mod shaders;