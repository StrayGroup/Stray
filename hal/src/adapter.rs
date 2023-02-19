use pollster::block_on;

pub struct SAdapter(wgpu::Adapter);

impl SAdapter{
    pub fn with(adapter: wgpu::Adapter) -> Self{
        Self(adapter)
    }

    pub fn raw(&self) -> &wgpu::Adapter{
        &self.0
    }

    pub fn request_device(&self) -> (crate::SDevice, crate::SQueue){
        let (device, queue)= block_on(
            self.raw().request_device(
                &wgpu::DeviceDescriptor {
                    features: self.raw().features(),
                    limits: wgpu::Limits::default(),
                    label: Some("Device"),
                },
                None,
            )
        ).unwrap();
        (crate::SDevice::with(device), crate::SQueue::with(queue))
    }
}