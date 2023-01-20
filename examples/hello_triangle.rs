use stray::prelude::*;
use legion::*;


#[system(for_each)]
fn draw(draw: &mut ScreenDraw){
    let vertices = vec![
        Vertex::new(-500, -500), Vertex::new(500, -500), Vertex::new(-500, 500), 
    ];
    let color = Color{
        r: 255,
        g: 255,
        b: 100,
        a: 1.0,
    };
    let material = StandardMaterial::new(color);
    draw.set_vertices(vertices);
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