use wgpu::{
    *,
    util::DeviceExt
};

use legion::{
    Resources, 
    system
};
use pollster::block_on;
use winit::{
    window::Window, 
    dpi::PhysicalSize
};

use stray_scene::*;

mod render;
mod draw;
mod pipeline;
pub use pipeline::*;
pub use render::*;
pub use draw::*;


#[system(for_each)]
pub fn read_geometry(
    draw_component: &mut ScreenDraw, 
    #[resource] device: &EngineData<Device>,
    #[resource] config: &EngineData<SurfaceConfiguration>,
    #[resource] render_query: &mut RenderQuery
) {
    render_query.0.push(draw_component.to_render_object(&device.0, &config.0));
}

#[system(for_each)]
pub fn read_sprites(
    transform: &Transform2D,
    sprite: &Sprite,
    #[resource] pipeline: &StrayTextureRenderPipeline,
    #[resource] device: &EngineData<Device>,
    #[resource] config: &EngineData<SurfaceConfiguration>,
    #[resource] queue: &EngineData<Queue>,
    #[resource] render_query: &mut RenderQuery
){
    render_query.0.push(sprite.to_render_object(&device.0, &config.0, &queue.0, &pipeline.1, transform));
}


// Render redraw as legion system
// executed in render_schedule

#[system]
pub fn redraw(
    #[resource] surface: &EngineData<Surface>, 
    #[resource] device: &EngineData<Device>,
    #[resource] shape_pipeline: &StrayShapeRenderPipeline,
    #[resource] texture_pipeline: &StrayTextureRenderPipeline,
    #[resource] queue: &EngineData<Queue>,
    #[resource] render_query: &mut RenderQuery,
) {
    let output = surface.0.get_current_texture().unwrap();
    let view = output.texture.create_view(&TextureViewDescriptor::default());
    let mut encoder = device.0.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    for entry in render_query.0.iter(){
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
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
    render_query.0.clear();
    queue.0.submit(std::iter::once(encoder.finish()));
    output.present();
}    



// Initializing render and write data into resources as Engine Data

pub fn initialize_render(res: &mut Resources, window: &Window, backend: StrayBackend) -> Result<(), &'static str>{
    let instance = match backend{
        StrayBackend::Vulkan => Instance::new(Backends::VULKAN),
        StrayBackend::Metal => Instance::new(Backends::METAL),
        StrayBackend::DX12 => Instance::new(Backends::DX12),
        _ => Instance::new(Backends::all())
    };

    let adapters = match backend{
        StrayBackend::Vulkan => instance.enumerate_adapters(Backends::VULKAN),
        StrayBackend::Metal => instance.enumerate_adapters(Backends::METAL),
        StrayBackend::DX12 => instance.enumerate_adapters(Backends::DX12),
        _ => instance.enumerate_adapters(Backends::all())
    };
    let adapter = adapters.into_iter().next().unwrap();
    let surface = unsafe {instance.create_surface(&window)};
    let adapter_features = adapter.features();
    let (device, queue) = block_on(request_device(&adapter, adapter_features));
    let window_size = window.inner_size();
    let config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: surface.get_supported_formats(&adapter)[0],
        width: window_size.width,
        height: window_size.height,
        present_mode: PresentMode::Mailbox,
        alpha_mode: CompositeAlphaMode::Auto,
    };

    // Pipeline creation, see pipeline/mod.rs
    let shape_pipeline = create_shape_pipeline(&device, &config);
    let texture_pipeline = create_texture_pipeline(&device, &config);
    
    surface.configure(&device, &config);
    println!("Using {} ({:?})", adapter.get_info().name, adapter.get_info().backend);

    // Insert all of these struct as Engine Data
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
