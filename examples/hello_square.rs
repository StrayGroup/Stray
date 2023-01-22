use stray::prelude::*;
use legion::system;


#[system(for_each)]
fn draw(draw: &mut ScreenDraw){
    let vertices = vec![
        Vertex::new(-500, -500), Vertex::new(500, -500), Vertex::new(-500, 500), Vertex::new(500, 500)
    ];

    let indices = vec![
        0, 1, 2,
        2, 1, 3
    ];

    let color = StrayColor{
        r: 255,
        g: 255,
        b: 100,
        a: 0.5,
    };
    let material = StandardMaterial::new(color);
    draw.set_vertices(vertices);
    draw.set_indices(indices);
    draw.set_material(material);
}

fn main(){
    Stray::new()
        .with_title("Stray App")
        .push((ScreenDraw::init(),))
        .add_system(draw_system())
        .build()
        .run();
        
}