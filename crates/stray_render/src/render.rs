use winit::window::Window;

use crate::RawVertex;

pub enum StrayBackend{
    Vulkan,
    Metal,
    DX12,
    All
}

impl Default for StrayBackend{
    fn default() -> Self {
        Self::All
    }
}

pub trait Renderer{
    fn init(backend: StrayBackend, window: &Window) -> Self;
    fn redraw(&self);
    fn set_vertex_buffer(&mut self, vertices: &[RawVertex]);
    fn set_indices_buffer(&mut self, indices: &[u16]);
}
