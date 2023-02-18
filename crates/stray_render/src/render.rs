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

use crate::hal::*;
use crate::{create_shape_pipeline, create_texture_pipeline};

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

pub fn initialize_render(res: &mut Resources, window: &Window, backend: &SBackend) -> Result<(), &'static str>{
    let instance = InstanceCreator::from(backend);
    let adapter = instance.request_adapter(backend);
    let surface = instance.create_surface(&window);

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
    let shape_pipeline = create_shape_pipeline(&device.raw(), &config.raw());
    let texture_pipeline = create_texture_pipeline(&device.raw(), &config.raw());
    //let smaa_target = SmaaTarget::new(&device, &queue, config.width, config.height, swapchain_format, smaa::SmaaMode::Smaa1X);
    surface.raw().configure(&device.raw(), &config.raw());
    println!("Stray Engine v0.1");
    println!("Using {} ({:?})", adapter.raw().get_info().name, adapter.raw().get_info().backend);

    // Insert all of these struct as Engine Data
    //res.insert(EngineData(smaa_target));
    res.insert(EngineData(instance));
    res.insert(EngineData(adapter ));
    res.insert(EngineData(surface ));
    res.insert(EngineData(device  ));
    res.insert(EngineData(queue   ));
    res.insert(EngineData(config  ));
    res.insert(EngineData(StrayIndexBuffer(None, 0)));
    res.insert(EngineData(StrayVertexBuffer(None, 0)));
    res.insert(shape_pipeline);
    res.insert(texture_pipeline);
    res.insert(RenderQuery(vec![]));

    Ok(())
}
