use legion::*;
use stray_scene::EngineData;
use wgpu::{Surface, Device, SurfaceConfiguration};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window}, dpi::{PhysicalSize, PhysicalPosition},
};

use stray_systems::*;
use stray_render::*;

mod settings;
mod plugin;
pub use plugin::*;
pub use settings::*;

pub struct Stray{
    pub global_schedule: Option<Schedule>,
    pub render_schedule: Option<Schedule>,
    pub global_resources: Resources,
    pub render_resources: Resources,
    pub plugins: Vec<Box<dyn Plugin>>,
    pub window: Window,
    pub event_loop: EventLoop<()>,
    pub world: World,
}

impl Stray{
    pub fn new() -> StrayBuilder{
        StrayBuilder::new()
    }

    pub fn get_resources(&self) -> &Resources{
        &self.global_resources
    }

    pub fn get_mut_resources(&mut self) -> &mut Resources{
        &mut self.global_resources
    }

    pub fn run(mut self) {
        self.global_resources.insert(InputEvent::NONE);
        self.global_resources.insert(LastState::NONE);
        let mut r_schedule = self.render_schedule.unwrap();
        let mut g_schedule = self.global_schedule.unwrap();
        match initialize_render(&mut self.render_resources, &self.window, StrayBackend::All){
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
                    WindowEvent::CursorMoved {position,.. } => {
                        // WIP
                        let true_position = [position.x-(self.window.inner_size().width/2) as f64, position.y-(self.window.inner_size().height/2) as f64];
                    }
                    WindowEvent::MouseInput {  state, button, .. } =>{
                        // WIP
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        self.global_resources.insert(InputEvent::from(input));
                        self.global_resources.insert(LastState::from(input));
                    },

                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        resize(&self.render_resources, &self.global_resources, *physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        resize(&self.render_resources, &self.global_resources, **new_inner_size);
                        
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
    
                },
                Event::MainEventsCleared => {
                    parse_settings(&self.global_resources.get::<Settings>().unwrap(), &self.window);
                    r_schedule.execute(&mut self.world, &mut self.render_resources);
                    g_schedule.execute(&mut self.world, &mut self.global_resources);
                    self.global_resources.insert(InputEvent::NONE);
                }
                _ => {}
        });
    }
}

pub struct StrayBuilder{
    pub render_schedule: systems::Builder,
    pub global_schedule: systems::Builder,
    pub once_schedule: systems::Builder,
    pub stray: Stray,
    pub settings: Settings,
}

impl StrayBuilder{
    pub fn new() -> Self{
        let global_schedule = Schedule::builder();
        let render_schedule = Schedule::builder();
        let once_schedule = Schedule::builder();
        let settings = Settings::default();
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_inner_size(PhysicalSize::new(600, 600))
            .build(&event_loop).unwrap();
        
        let stray = Stray { 
            global_schedule: None,  
            render_schedule: None,
            global_resources: Resources::default(), 
            render_resources: Resources::default(),
            plugins: vec![],
            window: window, 
            event_loop: event_loop, 
            world: World::default(),
        };

        Self { 
            render_schedule, 
            global_schedule,
            once_schedule,
            stray,
            settings,
        }
    }
    pub fn with_title(mut self, title: &'static str) -> Self{
        self.settings.title = title;
        self
    }
    pub fn with_size(mut self, width: u32, height: u32) -> Self{
        self.settings.width = width;
        self.settings.height = height;
        self
    }
    pub fn set_resources(mut self, resources: Resources) -> Self{
        self.stray.global_resources = resources;
        self
    }

    pub fn run_once<R>(mut self, system: R) -> Self
    where
        R: systems::ParallelRunnable + 'static
    {
        self.once_schedule.add_system(system);
        self
    }
    pub fn add_system<S>(mut self, system: S) -> Self
    where
        S: systems::ParallelRunnable + 'static
    {
        self.global_schedule.add_system(system);
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
        self.stray.global_resources.insert(res);
        self
    }

    pub fn add_plugin<P>(mut self, plugin: P, resources: &mut Resources) -> Self
    where
        P: Plugin + 'static
    {
        plugin.build(self, resources)
    }

    pub fn init_systems(&mut self){
        self.render_schedule.add_system(read_geometry_system());
        self.render_schedule.add_system(redraw_system());
        self.render_schedule.add_system(read_sprites_system());
    }
    
    pub fn build(mut self) -> Stray{
        self.init_systems();
        parse_settings(&self.settings,&self.stray.window);
        self.stray.global_resources.insert(self.settings);
        self.once_schedule.build().execute(&mut self.stray.world, &mut self.stray.global_resources); 
        self.stray.global_schedule = Some(self.global_schedule.build());
        self.stray.render_schedule = Some(self.render_schedule.build());
        self.stray
    }


}


// Other stuff

pub fn resize(render_res: &Resources, global_res: &Resources, new_size: winit::dpi::PhysicalSize<u32>) {
    if new_size.width > 0 && new_size.height > 0 {
        global_res.get_mut::<Settings>().unwrap().width = new_size.width;
        global_res.get_mut::<Settings>().unwrap().height = new_size.height;
        render_res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.width = new_size.width;
        render_res.get_mut::<EngineData<SurfaceConfiguration>>().unwrap().0.height = new_size.height;
        render_res.get::<EngineData<Surface>>().unwrap().0.configure(&render_res.get::<EngineData<Device>>().unwrap().0, &render_res.get::<EngineData<SurfaceConfiguration>>().unwrap().0);
    }
}

// WIP
pub enum MouseEvent{
    ENTERED,
    MOVED(PhysicalPosition<i64>)
}

pub type Key = VirtualKeyCode;

pub enum LastState{
    PRESSED(Key),
    RELEASED(Key),
    NONE
}
pub enum InputEvent{
    PRESSED(Key),
    RELEASED(Key),
    NONE
}

impl InputEvent{
    pub fn is_pressed(&self, vk: Key) -> bool{
        match self{
            Self::PRESSED(key) if key == &vk =>  true,
            _ => false
        }
    }

    pub fn is_released(&self, vk: Key) -> bool{
        match self{
            Self::RELEASED(key) if key == &vk =>  true,
            _ => false
        }
    }
}

impl From<&KeyboardInput> for InputEvent{
    fn from(value: &KeyboardInput) -> Self {
        match value.state{
            ElementState::Pressed => Self::PRESSED(value.virtual_keycode.unwrap()),
            ElementState::Released => Self::RELEASED(value.virtual_keycode.unwrap())
        }
    }
}


impl LastState{
    pub fn was_pressed(&self, vk: Key) -> bool{
        match self{
            Self::PRESSED(key) if key == &vk =>  true,
            _ => false
        }
    }

    pub fn was_released(&self, vk: Key) -> bool{
        match self{
            Self::RELEASED(key) if key == &vk =>  true,
            _ => false
        }
    }
}

impl From<&KeyboardInput> for LastState{
    fn from(value: &KeyboardInput) -> Self {
        match value.state{
            ElementState::Pressed => Self::PRESSED(value.virtual_keycode.unwrap()),
            ElementState::Released => Self::RELEASED(value.virtual_keycode.unwrap())
        }
    }
}