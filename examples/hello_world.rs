use stray::prelude::*;
use legion::*;


#[system(for_each)]
fn draw(draw: &mut Draw){
    let vertices = vec![
        Vertex::new(-10, -10), Vertex::new(10, -10), Vertex::new(-10, 10),
    ];
    draw.set_vertices(vertices);
}

fn main(){
    let mut stray = Stray::new(World::default());
    let settings = Settings::with_title("Stray App", Backend::Vulkan); 
    stray.world.push((Draw::init(),));
    stray.add_system(draw_system());
    stray.run(&settings);
}