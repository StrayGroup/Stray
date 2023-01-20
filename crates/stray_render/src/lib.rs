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

use stray_scene::EngineData;

mod render;
mod draw;
mod pipeline;
pub use pipeline::*;
pub use render::*;
pub use draw::*;


// For texture rendering WIP

// fn set_bind_groups(textures: &Container<Sprite>, resources: Resources)  -> EngineData<Vec<BindGroup>>{
//     let txt_storage = resources.get::<TexturesStorage>().unwrap();
    
//     for i in textures.0.iter(){
//         let texture = match txt_storage.get_from_id(i.texture_id){
//             Some(value) => value,
//             None => {
//                 eprintln!("Incorred texture id in sprite: {:?}", i);
//                 std::process::exit(1);
//             },
//         };
        
//     }
//     EngineData(vec![])
// }

#[system(for_each)]
pub fn read_geometry(
    draw_component: &ScreenDraw, 
    #[resource] vertex: &mut EngineData<StrayVertexBuffer>,
    #[resource] index: &mut EngineData<StrayIndicesBuffer>,
    #[resource] device: &EngineData<Device>,
    #[resource] config: &EngineData<SurfaceConfiguration>,
) {
    let raw_size = [config.0.width as i32,config.0.height as i32];
    let vertex_array: &Vec<RawVertex> = &draw_component.vertices.iter().map(|x| x.to_raw(raw_size)).collect();
    let vertex_buffer = set_vertex_buffer(&device.0, &vertex_array.as_slice());
    vertex.set(vertex_buffer);
    let index_array: &Vec<u16> = &draw_component.indices;
    let index_buffer = set_indices_buffer(&device.0, &index_array.as_slice());
    index.set(index_buffer);
}



// Render redraw as legion system
// executed in render_schedule

#[system]
pub fn redraw(
    #[resource] surface: &EngineData<Surface>, 
    #[resource] device: &EngineData<Device>,
    #[resource] pipeline: &EngineData<RenderPipeline>,
    #[resource] vertex: &EngineData<StrayVertexBuffer>,
    #[resource] index: &EngineData<StrayIndicesBuffer>,
    #[resource] queue: &EngineData<Queue>,
) {
    let output = surface.0.get_current_texture().unwrap();
    let view = output.texture.create_view(&TextureViewDescriptor::default());
    let mut encoder = device.0.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });
    let v_buff = &vertex.0.0.as_ref().unwrap();
    let i_buff = &index.0.0.as_ref().unwrap();
    {
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
        render_pass.set_pipeline(&pipeline.0);
        render_pass.set_vertex_buffer(0, v_buff.slice(..));
        if index.0.1 > 0{
            render_pass.set_index_buffer(i_buff.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..index.0.1, 0, 0..1); 
        } else {
            render_pass.draw(0..vertex.0.1,0..1);
        }
        
    }
    
    queue.0.submit(std::iter::once(encoder.finish()));
    output.present();
}    

// Wgpu indices buffer wrapper for resource need

pub struct StrayIndicesBuffer(Option<Buffer>,u32);


// Set indices buffer, returns Engine Data 
// because it cannot borrow mutable resources 
// and immutable resources in the same time

pub fn set_indices_buffer(device: &Device, indices: &[u16]) -> StrayIndicesBuffer{
    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Indices Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        }
    );
    let index_buffer_len = indices.len() as u32;
    StrayIndicesBuffer(Some(index_buffer), index_buffer_len)
}

// Wgpu vertex buffer wrapper for resource need

pub struct StrayVertexBuffer(Option<Buffer>, u32);


// Set vertex buffer, returns Engine Data 
// because it cannot borrow mutable resources 
// and immutable resources in the same time

pub fn set_vertex_buffer(device: &Device, vertices: &[RawVertex]) -> StrayVertexBuffer{
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );
    let vertex_buffer_len = vertices.len() as u32;
    StrayVertexBuffer(Some(vertex_buffer), vertex_buffer_len)
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
        present_mode: PresentMode::Fifo,
        alpha_mode: CompositeAlphaMode::Auto,
    };

    // Pipeline creation, see pipeline/mod.rs
    let pipeline = create_pipeline(&device, &config);

    surface.configure(&device, &config);
    println!("Using {} ({:?})", adapter.get_info().name, adapter.get_info().backend);

    // Insert all of these struct as Engine Data
    res.insert(EngineData(instance));
    res.insert(EngineData(adapter));
    res.insert(EngineData(surface));
    res.insert(EngineData(device));
    res.insert(EngineData(queue));
    res.insert(EngineData(config));
    res.insert(EngineData(pipeline));
    res.insert(EngineData(StrayIndicesBuffer(None, 0)));
    res.insert(EngineData(StrayVertexBuffer(None, 0)));

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
