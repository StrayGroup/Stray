mod render;
mod draw;
pub use render::*;
pub use draw::*;
use legion::*;

#[system(for_each)]
pub fn draw(draw: &Draw, #[resource] render: &WgpuRender){
    println!("{:?}", draw.vertices);
}