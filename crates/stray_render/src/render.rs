
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

