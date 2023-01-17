use wgpu::{*, util::DeviceExt};
use pollster::block_on;
use winit::window::Window;

mod render;
mod draw;
mod pipeline;
pub use pipeline::*;
pub use render::*;
pub use draw::*;

pub struct WgpuRender{
    instance: Instance,
    adapter: Adapter,
    surface: Surface,
    device: Device,
    queue: Queue,
    pub config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    vertex_buffer: Option<Buffer>,
    vertex_buffer_len: u32,
    index_buffer: Option<Buffer>,
    index_buffer_len: u32,
}

impl Renderer for WgpuRender{
    fn redraw(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let v_buff = self.vertex_buffer.as_ref().unwrap();
        let i_buff = self.index_buffer.as_ref().unwrap();
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
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, v_buff.slice(..));
            if self.index_buffer_len > 0{
                render_pass.set_index_buffer(i_buff.slice(..), IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.index_buffer_len, 0, 0..1); 
            } else {
                render_pass.draw(0..self.vertex_buffer_len,0..1);
            }
            
        }
        
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    fn init(backend: StrayBackend, window: &Window) -> Self{
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
        let pipeline = create_pipeline(&device, &config);
        let vertex_buffer = None;
        let vertex_buffer_len = 0;
        let index_buffer = None;
        let index_buffer_len = 0;
        surface.configure(&device, &config);
        println!("Using {} ({:?})", adapter.get_info().name, adapter.get_info().backend);
        Self{
            instance, 
            adapter, 
            device, 
            queue, 
            surface, 
            config, 
            pipeline, 
            vertex_buffer, 
            vertex_buffer_len,
            index_buffer,
            index_buffer_len,
        }


    }
    fn set_vertex_buffer(&mut self, vertices: &[RawVertex]) {
        self.vertex_buffer = Some(self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        ));
        self.vertex_buffer_len = vertices.len() as u32;
    }
    fn set_indices_buffer(&mut self, indices: &[u16]) {
        self.index_buffer = Some(self.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Indices Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }
        ));
        self.index_buffer_len = indices.len() as u32;
    }
}

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
