//use smaa::{SmaaTarget, SmaaFrame};
use wgpu::{
    *,
    util::DeviceExt
};

use glam::*;

use hal::*;

use legion::{
    Resources, 
};

use winit::{
    window::Window, 
};

use crate::Rendy;
use crate::RenderObject;
use crate::RenderState;

use stray_scene::*;


// Initializing render and write data into resources as Engine Data

pub fn initialize_render(res: &mut Resources, window: &Window, backend: &SBackend) -> Result<(), &'static str>{
    let instance = InstanceCreator::from(backend);
    let adapter = instance.request_adapter(backend);
    let surface = instance.create_surface(window);

    let (device, queue) = adapter.request_device();
    let window_size = window.inner_size();
    let config = SConfig::with(
        window_size.width,
        window_size.height,
        true,
        TextureFormat::Rgba8UnormSrgb
    );
    // let swapchain_format = surface.get_supported_formats(&adapter)[0];

    // Pipeline creation, see pipeline/mod.rs
    //let smaa_target = SmaaTarget::new(&device, &queue, config.width, config.height, swapchain_format, smaa::SmaaMode::Smaa1X);
    surface.raw().configure(&device.raw(), &config.raw());
    println!("Stray Engine v0.1");
    println!("Using {} ({:?})", adapter.raw().get_info().name, adapter.raw().get_info().backend);

    let render_state = RenderState::empty();
    // Insert all of these struct as Engine Data
    //res.insert(EngineData(smaa_target));
    res.insert(instance);
    res.insert(adapter );
    res.insert(surface );
    res.insert(device  );
    res.insert(queue   );
    res.insert(config  );
    res.insert(render_state);
    
    Ok(())
}


// Render Sprites

impl Rendy for Sprite{
    fn get_id(&self) -> u32 {
        self.id
    }
    fn render(
        &self,
        device: &SDevice,
        config: &SConfig,
        queue: &SQueue,
        transform: &Transform2D
    ) -> RenderObject{
        let vertices = self.create_vertices(device, config, transform).unwrap();
        let pipeline= self.create_pipeline(device, config);
        let bind_group = self.create_bind_group(device, queue, &pipeline.layout().as_ref().unwrap()).unwrap();
        RenderObject{
            id: self.get_id(),
            pipeline: Some(pipeline),
            vertex: Some(vertices),
            index: None,
            bind_group: Some(bind_group)
        }
    }

    fn create_vertices(
        &self, 
        device: &SDevice,
        config: &SConfig,
        transform: &Transform2D
    ) -> Option<SVertexBuffer>{
        let th = (self.texture.dimensions.1 as f32 * transform.size.y) as i32 ;
        let tw = (self.texture.dimensions.0 as f32 * transform.size.x) as i32;
        let raw_size = [config.raw().width as i32,config.raw().height as i32];
        let vertices_data = vec![
            TextureVertex::new(-tw, -th, self.layer, 0.0, 1.0), 
            TextureVertex::new(tw, -th, self.layer, 1.0, 1.0), 
            TextureVertex::new(-tw, th, self.layer, 0.0, 0.0), 
            TextureVertex::new(-tw, th, self.layer, 0.0, 0.0), 
            TextureVertex::new(tw, -th, self.layer, 1.0, 1.0 ), 
            TextureVertex::new(tw, th, self.layer, 1.0, 0.0),
        ];

        // Apply transform scale and make position in [-1,1] range
        let true_transform = Transform2D { 
            position: transform.position/vec2((config.raw().width/2) as f32, (config.raw().height/2) as f32), 
            rotation: transform.rotation, 
            size: transform.size
        };
        
        let vertices: Vec<RawVertex> = vertices_data.iter().map(|x| x.to_raw(raw_size, &true_transform)).collect();
        let vertex_buffer = device.raw().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_len = vertices_data.len() as u32;
        Some(SVertexBuffer(Some(vertex_buffer), vertex_buffer_len))
    }
    
    fn create_bind_group(
        &self, 
        device: &SDevice,
        queue: &SQueue,
        layout: &BindGroupLayout
    ) -> Option<BindGroup>{
        Some(self.texture.write_texture(device.raw(), queue.raw(), layout))
    }
    
    fn create_pipeline(&self, 
        device: &SDevice, 
        config: &SConfig
    ) -> SRenderPipeline{
        let layout = device.raw().create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            multisampled: false,
                            view_dimension: TextureViewDimension::D2,
                            sample_type: TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            }
        );
    
        let shader = device.raw().create_shader_module(include_wgsl!("shaders/texture.wgsl"));
        let render_pipeline_layout = device.raw().create_pipeline_layout(
            &PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            }
        );
        let pipeline = device.raw().create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState{
                module: &shader,
                entry_point: "vs_main",
                buffers: &[RawVertex::desc()]
            },
            fragment: Some(FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.raw().format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let s_pipeline = SRenderPipeline::with(
            pipeline, 
            RenderMode::DRAW,
            Some(layout)
        );
        s_pipeline
    }
}

// Render Shapes

impl Rendy for Canvas{
    fn get_id(&self) -> u32 {
        self.id
    }

    fn render(
        &self,
        device: &SDevice,
        config: &SConfig,
        queue: &SQueue,
        transform: &stray_scene::Transform2D
    ) -> RenderObject {
        let pipeline = self.create_pipeline(device, config);
        let vertex = self.create_vertices(device, config, transform).unwrap();
        let index = self.create_indices(device).unwrap();
        RenderObject { 
            id: self.id, 
            pipeline: Some(pipeline), 
            bind_group: None, 
            vertex: Some(vertex), 
            index: Some(index)
        }
    }

    fn create_pipeline(
        &self, 
        device: &SDevice, 
        config: &SConfig
    ) -> SRenderPipeline {
        let shader = device.raw().create_shader_module(include_wgsl!("shaders/shape.wgsl"));
        let render_pipeline_layout = device.raw().create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }
        );
        let render_pipeline = device.raw().create_render_pipeline(&RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState{
                module: &shader,
                entry_point: "vs_main",
                buffers: &[RawVertex::desc()]
            },
            fragment: Some(FragmentState{
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.raw().format,
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
    
        });
        
        if self.indices.len() > 0 {
            return SRenderPipeline::with(render_pipeline, RenderMode::DRAW_INDEXED, None);
        }
        SRenderPipeline::with(render_pipeline, RenderMode::DRAW, None)
    }

    fn create_vertices(
        &self, 
        device: &SDevice,
        config: &SConfig,
        transform: &stray_scene::Transform2D
    ) -> Option<stray_scene::SVertexBuffer> {
        let raw_size = [config.raw().width as i32,config.raw().height as i32];
        let true_transform = Transform2D { 
            position: transform.position/vec2((config.raw().width/2) as f32, (config.raw().height/2) as f32), 
            rotation: transform.rotation, 
            size: transform.size 
        };
        let vertices: Vec<RawVertex> = self.vertices.iter().map(|x| x.to_raw(raw_size, true_transform)).collect();
        let vertex_buffer = device.raw().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_len = self.vertices.len() as u32;
        Some(SVertexBuffer(Some(vertex_buffer), vertex_buffer_len))
    }

    fn create_indices(
        &self,
        device: &SDevice,
    ) -> Option<stray_scene::SIndexBuffer> {
        let index_buffer = device.raw().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Indices Buffer"),
                contents: bytemuck::cast_slice(self.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let index_buffer_len = self.indices.len() as u32;
        Some(SIndexBuffer(Some(index_buffer), index_buffer_len))
    }
}