use legion::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    platform::run_return::{self, EventLoopExtRunReturn},
};

use stray_render::*;
mod settings;
pub use settings::*;

pub struct Stray{
    pub world: World,
    pub resources: Resources,
    schedule_builder: legion::systems::Builder
}

impl Stray{
    pub fn new(world: World) -> Self{
        let resources = Resources::default();
        let schedule_builder = Schedule::builder();
        Self { world, resources, schedule_builder}
    }

    pub fn add_system<T>(&mut self, system: T)
    where
        T: systems::ParallelRunnable + 'static
    {
        self.schedule_builder.add_system(system);
    }

    pub fn run(&mut self, settings: &Settings) {
        let mut event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        self.schedule_builder.add_system(stray_render::draw_system());
        let mut schedule = self.schedule_builder.build();
        parse_settings(settings,&window);
        let render = match settings.backend{
            Backend::DX12 => {WgpuRender::init(Backend::DX12, &window)}
            Backend::Metal => {WgpuRender::init(Backend::Metal, &window)}
            Backend::Vulkan => {WgpuRender::init(Backend::Vulkan, &window)}
            Backend::All => {WgpuRender::init(Backend::All, &window)}
        };
        self.resources.insert(render);
        schedule.execute(&mut self.world, &mut self.resources);
        loop{

        }
        // event_loop.run(move |event, _, control_flow| match event {
        //     Event::WindowEvent {
        //         ref event,
        //         window_id,
        //     } if window_id == window.id() => match event {
        //         WindowEvent::CloseRequested
        //         | WindowEvent::KeyboardInput {
        //             input:
        //                 KeyboardInput {
        //                     state: ElementState::Pressed,
        //                     virtual_keycode: Some(VirtualKeyCode::Escape),
        //                     ..
        //                 },
        //             ..
        //         } => *control_flow = ControlFlow::Exit,
        //         _ => {*control_flow = ControlFlow::Exit}
        //     },
        //     _ => {}
        // });
    }
}
