use legion::*;
use wgpu::*;

use stray_scene::*;
use stray_render::render_redraw;

// use smaa::SmaaTarget;

#[system(for_each)]
pub fn read_geometry(
    draw_component: &mut ScreenDraw, 
    #[resource] device: &EngineData<Device>,
    #[resource] config: &EngineData<SurfaceConfiguration>,
    #[resource] render_query: &mut RenderQuery
) {
    render_query.0.push(draw_component.to_render_object(&device.0, &config.0));
}

#[system(for_each)]
pub fn read_sprites(
    transform: &Transform2D,
    sprite: &Sprite,
    #[resource] pipeline: &StrayTextureRenderPipeline,
    #[resource] device: &EngineData<Device>,
    #[resource] config: &EngineData<SurfaceConfiguration>,
    #[resource] queue: &EngineData<Queue>,
    #[resource] render_query: &mut RenderQuery
){
    render_query.0.push(sprite.to_render_object(&device.0, &config.0, &queue.0, &pipeline.1, transform));
}

#[system]
pub fn redraw(
    #[resource] surface: &EngineData<Surface>, 
    #[resource] device: &EngineData<Device>,
    #[resource] shape_pipeline: &StrayShapeRenderPipeline,
    #[resource] texture_pipeline: &StrayTextureRenderPipeline,
    #[resource] queue: &EngineData<Queue>,
    #[resource] render_query: &mut RenderQuery,
   // #[resource] smaa_target: &mut EngineData<SmaaTarget>
){
    render_redraw(&surface.0, &device.0, &shape_pipeline, &texture_pipeline, &queue.0, render_query)
}