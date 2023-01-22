use wgpu::*;

use stray_scene::{StrayShapeRenderPipeline, StrayTextureRenderPipeline, RawVertex};

pub fn create_shape_pipeline(device: &Device, config: &SurfaceConfiguration) -> StrayShapeRenderPipeline{
    let shader = device.create_shader_module(include_wgsl!("shaders/shape.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(
        &wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        }
    );
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor{
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
                format: config.format,
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

    StrayShapeRenderPipeline(render_pipeline)
}


pub fn create_texture_pipeline(device: &Device, config: &SurfaceConfiguration) -> StrayTextureRenderPipeline{
    let layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
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
            });
    
    let shader = device.create_shader_module(include_wgsl!("shaders/texture.wgsl"));
    let render_pipeline_layout = device.create_pipeline_layout(
        &PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        }
    );
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor{
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
                format: config.format,
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

    StrayTextureRenderPipeline(render_pipeline, layout)
}



