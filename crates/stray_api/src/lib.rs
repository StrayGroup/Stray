use legion::*;
use stray_scene::EngineData;
use wgpu::{Surface, Device, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::PhysicalSize,
};

use stray_render::*;

mod settings;
use settings::*;

pub struct Stray{
    schedule: Option<Schedule>,
    resources: Resources,
    window: Window,
    event_loop: EventLoop<()>,
    world: World,
}

impl Stray{
    pub fn new() -> StrayBuilder{
        StrayBuilder::new()
    }
    pub fn run(mut self) {
        let mut g_schedule = self.schedule.unwrap();
        match initialize_render(&mut self.resources, &self.window, StrayBackend::All){
            Err(e) => {
                eprintln!("Render Error: {}", e);
                std::process::exit(1);
            }
            Ok(_) => {}
        }
        self.event_loop.run(move |event, _, control_flow| 
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        resize(&self.resources, *physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        resize(&self.resources, **new_inner_size);
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    
                },
                Event::MainEventsCleared => {
                    g_schedule.execute(&mut self.world, &mut self.resources);
                }
                _ => {}
        });
    }
}

pub struct StrayBuilder{
    schedule: systems::Builder,
    stray: Stray,
    settings: Settings,
}

impl StrayBuilder{
    pub fn new() -> Self{
        let schedule = Schedule::builder();
        let settings = Settings::default();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(600, 600))
            .build(&event_loop).unwrap();
        let stray = Stray { 
            schedule: None,  
            resources: Resources::default(), 
            window: window, 
            event_loop: event_loop, 
            world: World::default() 
        };

        Self { 
            schedule, 
            stray,
            settings,
        }
    }
    pub fn with_title(mut self, title: &str) -> Self{
        self.settings.title = title.to_string();
        self
    }

    pub fn add_system<S>(mut self, system: S) -> Self
    where
        S: systems::ParallelRunnable + 'static
    {
        self.schedule.add_system(system);
        self
    }

    pub fn push<T>(mut self, comp: T) ->  Self
    where
        Option<T>: storage::IntoComponentSource
    {
        self.stray.world.push(comp);
        self
    }

    pub fn insert<T>(mut self, res: T) -> Self
    where
        T: 'static
    {
        self.stray.resources.insert(res);
        self
    }

    pub fn init_systems(&mut self){
        self.schedule.add_system(read_geometry_system());
        self.schedule.add_system(redraw_system());
    }
    
    pub fn build(mut self) -> Stray{
        self.init_systems();
        parse_settings(&self.settings,&self.stray.window);
        self.stray.schedule = Some(self.schedule.build());
        self.stray
    }


}


// Other stuff

pub fn resize(res: &Resources, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
        res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.width = new_size.width;
        res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.height = new_size.height;
        res.get::<EngineData<Surface>>().unwrap().0.configure(&res.get::<EngineData<Device>>().unwrap().0, &res.get::<EngineData<SurfaceConfiguration>>().unwrap().0);
    }
}
