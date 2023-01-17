use wgpu::*;
use winit::window::Window;
use pollster::*;

use crate::RawVertex;

pub enum Backend{
    Vulkan,
    Metal,
    DX12,
    All
}

impl Default for Backend{
    fn default() -> Self {
        Self::All
    }
}

pub trait Renderer{
    fn init(backend: Backend, window: &Window) -> Self;
    fn redraw(&self);
    fn set_vertex_buffer(&mut self, vertices: &[RawVertex]);
}
