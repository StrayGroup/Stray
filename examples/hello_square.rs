use stray::prelude::*;
use legion::*;


#[system(for_each)]
fn draw(draw: &mut Draw){
    let vertices = vec![
        Vertex::new(-500, -500), Vertex::new(500, -500), Vertex::new(-500, 500), Vertex::new(500, 500)
    ];

    let indices = vec![
        0, 1, 2,
        2, 1, 3
    ];

    let color = Color{
        r: 255,
        g: 255,
        b: 100,
        a: 1.0,
    };
    let material = StandardMaterial::new(color);
    draw.set_vertices(vertices);
    draw.set_indices(indices);
    draw.set_material(material);
}

fn main(){
    let mut stray = Stray::new(World::default());
    let settings = Settings::with_title("Stray App", StrayBackend::All); 
    let entity = stray.world.push((Draw::init(),));
    let display = [entity];
    stray.display(&display);
    stray.add_system(draw_system());
    stray.run(&settings);
}