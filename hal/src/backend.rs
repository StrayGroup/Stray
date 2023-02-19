
#[derive(Debug, Clone, Copy)]
pub enum SBackend{
    DX11,
    DX12,
    OGL,
    VULKAN,
    METAL,
    ALL
}