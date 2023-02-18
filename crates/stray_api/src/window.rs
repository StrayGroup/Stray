pub struct SEventLoop{
    raw_loop: winit::EventLoop,
}

impl SEventLoop{
    pub fn run(self, stray: &mut crate::Stray){
        self.raw_loop.run(move |event, _, control_flow| {event_handler(
            stray,
            event,
            _,
            control_flow
        )});
    }
}

fn event_handler(
    stray: &mut crate::Stray, 
    event: winit::Event<()>, 
    window_target: winit::EventLoopWindowTarget,
    control_flow: &mut winit::ControlFlow
){

}


