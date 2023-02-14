use stray::prelude::*;
use legion::{
    *,
    world::SubWorld
};

mod ball;
mod paddles;

fn main(){
    Stray::new()
        .with_size(600, 400) // Set size of window
        .with_title("Pong!") // Set title of window

        .run_once(ball::init_ball_system()) // Add ball systems
        .add_system(ball::ball_logic_system())

        .run_once(paddles::init_paddles_system()) // Add paddle systems
        .add_system(paddles::left_paddle_movement_system())
        .add_system(paddles::right_paddle_movement_system())

        .build()
        .run();
}


// Collision checking algorithm TODO: make it built-in engine
pub fn check_aabb(
    mut first_aabb: [Vec2;2],
    first_transform: &Transform2D,
    mut second_aabb: [Vec2;2],
    second_transform: &Transform2D
) -> bool{
    first_aabb[0] += first_transform.position;
    first_aabb[1] += first_transform.position;
    second_aabb[0] += second_transform.position;
    second_aabb[1] += second_transform.position;

    first_aabb[1].x >= second_aabb[0].x && first_aabb[0].x <= second_aabb[1].x &&
    first_aabb[1].y >= second_aabb[0].y && first_aabb[0].y <= second_aabb[1].y 
}