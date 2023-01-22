use stray::prelude::*;
use legion::*;

#[system(for_each)]
fn rotating_sprite(transform: &mut Transform2D){
    transform.rotation += 5.0;
}

fn main(){
    Stray::new()
        .push((
            Transform2D::new(0.0, 0.0, 0.0),
            Sprite::new(include_bytes!("sprite.png"))
        ))
        .add_system(rotating_sprite_system())
        .build()
        .run();
}