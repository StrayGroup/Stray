use stray::prelude::*;
use legion::*;

#[system(for_each)]
fn rotating_sprite(transform: &mut Transform2D, #[resource] input: &InputEvent){
    if input.is_pressed(Key::A){
        transform.position.x -= 10.0;
    }
    if input.is_pressed(Key::D){
        transform.position.x += 10.0;
    }
    if input.is_pressed(Key::W){
        transform.position.y += 10.0;
    }
    if input.is_pressed(Key::S){
        transform.position.y -= 10.0;
    }
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