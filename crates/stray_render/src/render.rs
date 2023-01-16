use wgpu::*;
use winit::window::Window;
use pollster::*;

pub enum RenderApi{
    WGPU(WgpuRender, Backend)
}

pub enum Backend{
    Vulkan,
    Metal,
    DX12,
    All
}

impl Default for Backend{
    fn default() -> Self {
        Self::All
    }
}

pub trait Renderer{
    fn init(backend: Backend, window: &Window) -> Self;
    fn redraw(&self);
}


pub struct WgpuRender{
    instance: Instance,
    adapter: Adapter,
    surface: Surface,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration
}

impl Renderer for WgpuRender{
    fn redraw(&self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
         
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.9,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
                });
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
    fn init(backend: Backend, window: &Window) -> Self{
        let instance = match backend{
            Backend::Vulkan => Instance::new(Backends::VULKAN),
            Backend::Metal => Instance::new(Backends::METAL),
            Backend::DX12 => Instance::new(Backends::DX12),
            _ => Instance::new(Backends::all())
        };

        let adapters = match backend{
            Backend::Vulkan => instance.enumerate_adapters(Backends::VULKAN),
            Backend::Metal => instance.enumerate_adapters(Backends::METAL),
            Backend::DX12 => instance.enumerate_adapters(Backends::DX12),
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
        surface.configure(&device, &config);
        Self{instance, adapter, device, queue, surface, config}
    }
}

async fn request_device(adapter: &Adapter, adapter_features: Features) -> (Device, Queue){
    adapter.request_device(
        &DeviceDescriptor {
            features: adapter_features,
            limits: Limits::default(),
            label: Some(""),
        },
        None,
    ).await.unwrap()
}