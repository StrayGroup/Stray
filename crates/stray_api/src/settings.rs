use stray_render::StrayBackend;
use winit::dpi::PhysicalSize;


#[derive(Clone, Copy)]
pub struct Settings{
    pub always_on_top: bool,
    pub title: &'static str,
    pub resizable: bool,
    pub backend: StrayBackend,
    pub width: u32,
    pub height: u32,
}

impl Default for Settings{
    fn default() -> Self {
        Self { 
            always_on_top: false, 
            title: "Stray App", 
            resizable: true, 
            backend: StrayBackend::All,
            width: 600,
            height: 600
        }
    }
}

pub fn parse_settings(settings: &Settings, window: &winit::window::Window){
    window.set_always_on_top(settings.always_on_top);
    window.set_title(settings.title);
    window.set_resizable(settings.resizable);
    window.set_inner_size(PhysicalSize::new(settings.width, settings.height));

}