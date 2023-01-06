use stray::prelude::Stray;
use legion::World;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::ffi::CStr;
use std::os::raw::c_char;

lazy_static!{
    static ref WORLD: Mutex<World> = Mutex::new(World::default());
}

#[repr(C)]
pub struct stray_native{
    settings: &'static settings_native
}

#[repr(C)]
pub struct settings_native{
    pub title: *const c_char
}

#[repr(C)]
pub struct entity_native{
    components_ids: &'static [i32;1000]
}

// Functions For Initialize

#[no_mangle]
pub extern fn create_settings(title: *const c_char) -> settings_native{
    settings_native{title}
}

#[no_mangle]
pub extern fn create_stray(settings: &'static settings_native) -> stray_native{
    stray_native{settings}
}

#[no_mangle]
pub extern fn run_stray(stray: &'static stray_native){
    while true{}
}

// Functions For ECS

#[no_mangle]
pub extern fn add_entity_to_world(entity: entity_native){
    
}
