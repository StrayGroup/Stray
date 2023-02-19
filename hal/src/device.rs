pub struct SDevice(wgpu::Device);

impl SDevice{
    pub fn with(device: wgpu::Device) -> Self{
        Self(device)
    }

    pub fn raw(&self) -> &wgpu::Device{
        &self.0
    }
}

pub struct SQueue(wgpu::Queue);

impl SQueue{
    pub fn with(queue: wgpu::Queue) -> Self{
        Self(queue)
    }

    pub fn raw(&self) -> &wgpu::Queue{
        &self.0
    }
}