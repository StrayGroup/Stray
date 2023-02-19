use legion::{*, storage::Component};
use wgpu::*;
use hal::*;

use stray_scene::*;
use stray_render::*;

// use smaa::SmaaTarget;

#[system(for_each)]
pub fn convert_to_render_objects<OBJ: Component + Rendy>(
    object: &OBJ,
    transform: &Transform2D,
    #[resource] device: &SDevice,
    #[resource] config: &SConfig,
    #[resource] queue: &SQueue,
    #[resource] render: &mut RenderState
){
    if render.exist(object.get_id()){
        render.find(object.get_id()).update(object, device, config, queue, transform);
        return;
    } 
    render.push(object.render(device, config, queue, transform));
}
#[system]
pub fn redraw(
    #[resource] surface: &SSurface, 
    #[resource] device: &SDevice,
    #[resource] queue: &SQueue,
    #[resource] render: &mut RenderState,
   // #[resource] smaa_target: &mut EngineData<SmaaTarget>
){
    render.redraw(surface, device , queue)
}