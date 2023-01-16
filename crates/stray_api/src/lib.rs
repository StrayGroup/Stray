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

pub struct Stray{
    pub world: World,
    pub resources: Resources,
    pub render: Option<WgpuRender>,
    schedule_builder: legion::systems::Builder
}

impl Stray{
    pub fn new(world: World) -> Self{
        let resources = Resources::default();
        let schedule_builder = Schedule::builder();
        let render = None;
        Self { world, resources, render, schedule_builder}
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
        let render = match settings.backend{
            Backend::DX12 => {WgpuRender::init(Backend::DX12, &window)}
            Backend::Metal => {WgpuRender::init(Backend::Metal, &window)}
            Backend::Vulkan => {WgpuRender::init(Backend::Vulkan, &window)}
            Backend::All => {WgpuRender::init(Backend::All, &window)}
        };
        schedule.execute(&mut self.world, &mut self.resources);
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
}
