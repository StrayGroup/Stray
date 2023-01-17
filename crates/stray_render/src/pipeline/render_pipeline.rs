use wgpu::*;

use crate::draw::RawVertex;

pub fn create_pipeline(device: &Device, config: &SurfaceConfiguration) -> RenderPipeline{
    let shader = device.create_shader_module(include_wgsl!("shaders/shader.wgsl"));
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

    render_pipeline
}    



