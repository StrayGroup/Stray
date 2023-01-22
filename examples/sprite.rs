use stray::prelude::*;

fn main(){
    Stray::new()
        .push((Sprite::new(include_bytes!("sprite.png")),))
        .build()
        .run();
}