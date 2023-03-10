#[cfg(not(feature = "no-default-features"))]
pub mod prelude{
    pub use winit::window::Window;
    pub use glam::*;
    pub use stray_render::*;
    pub use stray_material::*;
    pub use stray_api::*;
    pub use stray_texture::*;
    pub use stray_scene::*;
}

