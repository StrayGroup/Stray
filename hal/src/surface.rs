pub struct SSurface(wgpu::Surface);

impl SSurface{
    pub fn with(surface: wgpu::Surface) -> Self{
        Self(surface)
    }
    pub fn raw(&self) -> &wgpu::Surface{
        &self.0
    }
}

pub struct SConfig(wgpu::SurfaceConfiguration);

impl SConfig{
    pub fn with(width: u32, height: u32, vsync: bool, texture_format: wgpu::TextureFormat) -> Self{
        let config = wgpu::SurfaceConfiguration{
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: width,
            height: height,
            present_mode: match vsync{
                false => wgpu::PresentMode::AutoNoVsync,
                true => wgpu::PresentMode::AutoVsync
            },
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![texture_format]
        };
        Self(config)
    }
    pub fn raw(&self) -> &wgpu::SurfaceConfiguration{
        &self.0
    }

    pub fn raw_mut(&mut self) -> &mut wgpu::SurfaceConfiguration{
        &mut self.0
    }
}