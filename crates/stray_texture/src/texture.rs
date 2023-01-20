use image::*;


// WIP

pub struct TexturesStorage{
    pub textures: Vec<(i32, StrayTexture)>
}

impl TexturesStorage{
    pub fn with(id: i32, texture: StrayTexture) -> Self{
        Self{textures: vec![(id, texture)]}
    }

    pub fn get_from_id(&self, id: i32) -> Option<&StrayTexture>{
        for i in &self.textures{
            if i.0 == id{
                return Some(&i.1);
            }
        }
        None
    }
}

pub struct StrayTexture{
    rgba: RgbaImage,
    dimension: (u32,u32)
}

impl StrayTexture{
    pub fn with(rgba: RgbaImage, dimension: (u32,u32)) -> Self{
        Self{rgba,dimension}
    }

    pub fn write_texture(&self, device: wgpu::Device, queue: wgpu::Queue, layout: wgpu::BindGroupLayout) -> wgpu::BindGroup{
        let texture_size = wgpu::Extent3d {
            width: self.dimension.0,
            height: self.dimension.1,
            depth_or_array_layers: 1,
        };
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
            }
        );
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &self.rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.dimension.0),
                rows_per_image: std::num::NonZeroU32::new(self.dimension.1),
            },
            texture_size,
        );

        let texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });


        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&sampler),
                    }
                ],
                label: Some("bind_group"),
            }
        );

        bind_group
    }
}

pub fn load_texture(id: i32, resources: &mut legion::Resources, bytes: &[u8]){
    let bytes = bytes;
    let image = load_from_memory(bytes).unwrap();
    let rgba = image.to_rgba8();
    let dimensions = image.dimensions();
    resources.get_mut::<TexturesStorage>().expect("Unintialized Texture storage").textures.append(&mut vec![(id, StrayTexture::with(rgba, dimensions))]);

}


