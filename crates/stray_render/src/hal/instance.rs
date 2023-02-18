pub struct InstanceCreator;

impl InstanceCreator{
    pub fn from(backend: &crate::SBackend) -> SInstance{
        let instance = match backend{
            crate::SBackend::VULKAN => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends:  wgpu::Backends::VULKAN,
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
            crate::SBackend::METAL => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends:  wgpu::Backends::METAL,
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
            crate::SBackend::DX12 => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends:  wgpu::Backends::DX12,
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
            crate::SBackend::DX11 => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends:  wgpu::Backends::VULKAN,
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
            crate::SBackend::OGL => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends:  wgpu::Backends::GL,
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
            crate::SBackend::ALL => {wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler:  wgpu::Dx12Compiler::Fxc,
            })},
        }; // End

        SInstance::with(instance)
    }
}

pub struct SInstance(wgpu::Instance);

impl SInstance{
    fn with(instance: wgpu::Instance) -> Self{
        Self(instance)
    }

    pub fn raw(&self) -> &wgpu::Instance{
        &self.0
    }

    pub fn request_adapter(&self, backend: &crate::SBackend) -> crate::SAdapter{
        let adapters = match backend{
            crate::SBackend::VULKAN => self.raw().enumerate_adapters(wgpu::Backends::VULKAN),
            crate::SBackend::METAL => self.raw().enumerate_adapters(wgpu::Backends::METAL),
            crate::SBackend::DX12 => self.raw().enumerate_adapters(wgpu::Backends::DX12),
            crate::SBackend::DX11 => self.raw().enumerate_adapters(wgpu::Backends::DX11),
            crate::SBackend::OGL => self.raw().enumerate_adapters(wgpu::Backends::GL),
            crate::SBackend::ALL => self.raw().enumerate_adapters(wgpu::Backends::all())
        }; 
        let adapter = adapters.into_iter().next().unwrap();
        crate::SAdapter::with(adapter)
    }

    pub fn create_surface(&self, window: &winit::window::Window) -> crate::SSurface{
        let surface = unsafe{
            self.raw().create_surface(window).unwrap()
        };
        crate::SSurface::with(surface)
    }
}