use stray::prelude::*;
use legion::{*, systems::CommandBuffer};

pub const PADDLE_AABB: [Vec2;2] = [vec2(-7.0,-56.5), vec2(7.0, 56.5)];
pub struct LeftPaddle;
pub struct RightPaddle;

#[system]
pub fn init_paddles(cmd: &mut CommandBuffer, #[resource] window: &Window){
    cmd.push((
        LeftPaddle,
        Transform2D::new(
            -((window.inner_size().width as i32)/2), // TODO: Create method for getting window edge
            0,
            0,
            0.5
        ),
        Sprite::new(include_bytes!("textures/paddle.png"), 0)
    )); // Initialize Left Paddle

    cmd.push((
        RightPaddle,
        Transform2D::new(
            (window.inner_size().width as i32)/2, // TODO: Create method for getting window edge
            0,
            0,
            0.5
        ),
        Sprite::new(include_bytes!("textures/paddle.png"), 0)
    )); // Initialize Left Paddle
}

#[system(for_each)]
pub fn left_paddle_movement(
    left_paddle: &LeftPaddle,
    texture: &Sprite,
    transform: &mut Transform2D,

    #[resource] window: &Window,
    #[resource] input: &InputEvent
){
    if input.is_pressed(Key::W) && 
        (transform.position.y + ((texture.texture.dimensions.1/2) as f32)*0.5) < // TODO: create method for getting 
        ((window.inner_size().height as i32)/2) as f32                           // window edges and textures
    {
        transform.position.y += 20.0;
    } else if input.is_pressed(Key::S) && 
        (transform.position.y - ((texture.texture.dimensions.1/2) as f32)*0.5) > // TODO: create method for getting
        -((window.inner_size().height as i32)/2) as f32                          // window edges and textures
    {
        transform.position.y -= 20.0; // Move Paddle
    }

    transform.position.x = -((window.inner_size().width as i32)/2) as f32 // Set transform position always on window edge

}

#[system(for_each)]
pub fn right_paddle_movement(
    left_paddle: &RightPaddle,
    texture: &Sprite,
    transform: &mut Transform2D,

    #[resource] window: &Window,
    #[resource] input: &InputEvent
){
    if input.is_pressed(Key::Up) && 
        (transform.position.y + ((texture.texture.dimensions.1/2) as f32)*0.5) < // Same as previous paddle
        ((window.inner_size().height as i32)/2) as f32
    {
        transform.position.y += 20.0;
    } else if input.is_pressed(Key::Down) && 
        (transform.position.y - ((texture.texture.dimensions.1/2) as f32)*0.5) > // Same as previous paddle
        -((window.inner_size().height as i32)/2) as f32
    {
        transform.position.y -= 20.0;   // Move paddle
    }

    transform.position.x = ((window.inner_size().width as i32)/2) as f32 // Set transform position always on window edge
}