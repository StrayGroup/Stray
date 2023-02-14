//use smaa::{SmaaTarget, SmaaFrame};
use wgpu::*;

use legion::{
    Resources, 
};
use pollster::block_on;
use winit::{
    window::Window, 
};

use stray_scene::*;

use crate::{create_shape_pipeline, create_texture_pipeline};


#[derive(Clone, Copy)]
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


pub fn render_redraw(
    surface: &Surface, 
    device: &Device,
    shape_pipeline: &StrayShapeRenderPipeline,
    texture_pipeline: &StrayTextureRenderPipeline,
    queue: &Queue,
    render_query: &mut RenderQuery,
   // smaa_target: &mut SmaaTarget
) {
    let output = surface.get_current_texture().unwrap();
    let view = output.texture.create_view(&TextureViewDescriptor::default());
    //let smaa_frame = smaa_target.start_frame(&device, &queue, &view);
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    {
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                //view: &*smaa_frame,
                view: &view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color {
                        r: 0.06,
                        g: 0.06,
                        b: 0.06,
                        a: 1.0,
                    }),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });
        for entry in render_query.0.iter(){
            if &entry.type_id == &0{
                let vertex = entry.get_vertex();
                let index = entry.get_index();
                let v_buff = vertex.0.as_ref().unwrap();
                let i_buff = index.0.as_ref().unwrap();

                render_pass.set_pipeline(&shape_pipeline.0);
                render_pass.set_vertex_buffer(0, v_buff.slice(..));
                if index.1 > 0{
                    render_pass.set_index_buffer(i_buff.slice(..), IndexFormat::Uint16);
                    render_pass.draw_indexed(0..index.1, 0, 0..1); 
                } else {
                    render_pass.draw(0..vertex.1,0..1);
                }
                
            }

            if &entry.type_id == &1{
                let vertex = entry.get_vertex();
                let v_buff = vertex.0.as_ref().unwrap();
                render_pass.set_pipeline(&texture_pipeline.0);
                render_pass.set_bind_group(0, &entry.bind_group.as_ref().unwrap(), &[]);
                render_pass.set_vertex_buffer(0, v_buff.slice(..));
                render_pass.draw(0..vertex.1, 0..1);

            }
            
        }
    }
    render_query.0.clear();
    //smaa_frame.resolve();
    queue.submit(std::iter::once(encoder.finish()));
    output.present();
}    



// Initializing render and write data into resources as Engine Data

pub fn initialize_render(res: &mut Resources, window: &Window, backend: StrayBackend) -> Result<(), &'static str>{
    let instance = match backend{
        StrayBackend::Vulkan => {Instance::new( InstanceDescriptor {
            backends:  Backends::VULKAN,
            dx12_shader_compiler:  Dx12Compiler::Fxc,
       })},
        StrayBackend::Metal => {Instance::new( InstanceDescriptor {
            backends:  Backends::METAL,
            dx12_shader_compiler:  Dx12Compiler::Fxc,
       })},
        StrayBackend::DX12 => {Instance::new( InstanceDescriptor {
            backends:  Backends::DX12,
            dx12_shader_compiler:  Dx12Compiler::Fxc,
       })},
        _ => {Instance::new( InstanceDescriptor {
            backends: Backends::all(),
            dx12_shader_compiler:  Dx12Compiler::Fxc,
       })},
    };

    let adapters = match backend{
        StrayBackend::Vulkan => instance.enumerate_adapters(Backends::VULKAN),
        StrayBackend::Metal => instance.enumerate_adapters(Backends::METAL),
        StrayBackend::DX12 => instance.enumerate_adapters(Backends::DX12),
        _ => instance.enumerate_adapters(Backends::all())
    };
    let adapter = adapters.into_iter().next().unwrap();
    let surface = unsafe {instance.create_surface(&window).unwrap()};
    let adapter_features = adapter.features();
    let (device, queue) = block_on(request_device(&adapter, adapter_features));
    let window_size = window.inner_size();
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: TextureFormat::Rgba8Unorm,
        width: window_size.width,
        height: window_size.height,
        present_mode: PresentMode::AutoVsync,
        alpha_mode: CompositeAlphaMode::Auto,
        view_formats: vec![TextureFormat::Rgba8UnormSrgb]
    };
    // let swapchain_format = surface.get_supported_formats(&adapter)[0];

    // Pipeline creation, see pipeline/mod.rs
    let shape_pipeline = create_shape_pipeline(&device, &config);
    let texture_pipeline = create_texture_pipeline(&device, &config);
    //let smaa_target = SmaaTarget::new(&device, &queue, config.width, config.height, swapchain_format, smaa::SmaaMode::Smaa1X);
    surface.configure(&device, &config);
    println!("Stray Engine v0.1");
    println!("Using {} ({:?})", adapter.get_info().name, adapter.get_info().backend);

    // Insert all of these struct as Engine Data
    //res.insert(EngineData(smaa_target));
    res.insert(EngineData(instance));
    res.insert(EngineData(adapter));
    res.insert(EngineData(surface));
    res.insert(EngineData(device));
    res.insert(EngineData(queue));
    res.insert(EngineData(config));
    res.insert(EngineData(StrayIndexBuffer(None, 0)));
    res.insert(EngineData(StrayVertexBuffer(None, 0)));
    res.insert(shape_pipeline);
    res.insert(texture_pipeline);
    res.insert(RenderQuery(vec![]));

    Ok(())
}

// Other stuff

async fn request_device(adapter: &Adapter, adapter_features: Features) -> (Device, Queue){
    adapter.request_device(
        &DeviceDescriptor {
            features: adapter_features,
            limits: Limits::default(),
            label: Some("Device"),
        },
        None,
    ).await.unwrap()
}


