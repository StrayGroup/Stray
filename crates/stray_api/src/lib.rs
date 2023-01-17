use legion::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use std::borrow::Cow;

use stray_render::*;
mod settings;
pub use settings::*;

pub struct Stray<'a>{
    pub world: World,
    pub resources: Resources,
    pub render: Option<WgpuRender>,
    pub display_elements: &'a [Entity],
    schedule_builder: legion::systems::Builder
}

impl <'a>Stray<'a>{
    pub fn new(world: World) -> Self{
        let resources = Resources::default();
        let schedule_builder = Schedule::builder();
        let render = None;
        let display_elements = &[];
        Self { world, resources, render, display_elements, schedule_builder}
    }

    pub fn add_system<T>(&mut self, system: T)
    where
        T: systems::ParallelRunnable + 'static
    {
        self.schedule_builder.add_system(system);
    }

    pub fn run(&mut self, settings: &Settings) {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        //self.schedule_builder.add_system(stray_render::draw_system());
        let mut schedule = self.schedule_builder.build();
        parse_settings(settings,&window);
        let mut render = match settings.backend{
            Backend::DX12 => {WgpuRender::init(Backend::DX12, &window)}
            Backend::Metal => {WgpuRender::init(Backend::Metal, &window)}
            Backend::Vulkan => {WgpuRender::init(Backend::Vulkan, &window)}
            Backend::All => {WgpuRender::init(Backend::All, &window)}
        };
        schedule.execute(&mut self.world, &mut self.resources);
        let raw_window_size = [render.config.height as i32, render.config.height as i32];
        for i in self.display_elements{
            if let Some(comp) = self.world.entry(*i){
                let raw_vertices: Vec<RawVertex> = comp.get_component::<Draw>().unwrap().vertices.iter().map(|c| c.to_raw(raw_window_size)).collect();
                render.set_vertex_buffer(raw_vertices.as_slice());
            }
        }
        event_loop.run(move |event, _, control_flow| 
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.id() => match event {
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

                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    
                    render.redraw();
                },
                _ => {}
        });
    }

    pub fn display(&mut self, entities: &'a [Entity]){
        self.display_elements = entities;
    }
}
