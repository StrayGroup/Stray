use stray::prelude::*;
use legion::*;

#[system]
fn input(#[resource] event: &InputEvent){
    if event.is_pressed(Key::A){
        println!("A is pressed");
    }
}

fn main(){
    Stray::new()
        .add_system(input_system())
        .build()
        .run()
}