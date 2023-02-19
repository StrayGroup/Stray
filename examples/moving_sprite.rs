use stray::prelude::*;
use legion::{*, systems::CommandBuffer};

#[system(for_each)]
fn rotating_sprite(
    transform: &mut Transform2D, 
    #[resource] input: &InputEvent,
){
    if input.is_pressed(Key::A){
        transform.position.x -= 20.0;
    }
    if input.is_pressed(Key::D){
        transform.position.x += 20.0;
    }
    if input.is_pressed(Key::W){
        transform.position.y += 20.0;
    }
    if input.is_pressed(Key::S){
        transform.position.y -= 20.0;
    }
    if input.is_pressed(Key::Space){
        transform.position *= -1.0;
    }
}

#[system]
fn setup_sprite(commands: &mut CommandBuffer){
    commands.push((
        Transform2D::new(vec2(0.0, 0.0),  0, vec2(1.0,1.0)),
        Sprite::new(0, include_bytes!("sprite.png"), 0)
    ));
}

fn main(){
    Stray::new()
        .with_size(1000, 500)
        .add_system(rotating_sprite_system())
        .run_once(setup_sprite_system())
        .build()
        .run();
}