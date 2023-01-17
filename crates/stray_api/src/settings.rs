use stray_render::StrayBackend;

#[derive(Default)]
pub struct Settings{
    pub always_on_top: bool,
    pub title: String,
    pub resizable: bool,
    pub backend: StrayBackend
}

impl Settings{
    pub fn with_title(title: &str, backend: StrayBackend) -> Self{
        let mut ret = Self::default();
        ret.title = title.to_string();
        ret.backend = backend;
        ret
    }
}


pub fn parse_settings(settings: &Settings, window: &winit::window::Window){
    window.set_always_on_top(settings.always_on_top);
    window.set_title(settings.title.as_str());
    window.set_resizable(settings.resizable);

}