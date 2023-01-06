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