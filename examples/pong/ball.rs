use stray::prelude::*;
use legion::{*, systems::CommandBuffer, world::SubWorld};

use crate::paddles::PADDLE_AABB;

const BALL_AABB: [Vec2;2] = [vec2(-7.0,-7.0), vec2(7.0,7.0)]; // Is scaled to 0.5
pub struct Ball{
    velocity: Vec2
}

#[system]
pub fn init_ball(cmd: &mut CommandBuffer){
    cmd.push((
        Ball{
            velocity: vec2(2.0,2.0)
        },
        Transform2D::new(
            0, 
            0, 
            0, 
            0.5
        ),
        Sprite::new(include_bytes!("textures/ball.png"), 0)
    )); // Initialize ball
}

#[system]
pub fn ball_logic(
    world: &mut SubWorld,

    ball_query: &mut Query<(&mut Transform2D, &mut Ball, &Sprite)>,
    left_paddle: &mut Query<(&Transform2D, &crate::paddles::LeftPaddle)>,
    right_paddle: &mut Query<(&Transform2D, &crate::paddles::RightPaddle)>,

    #[resource] window: &Window
){
    let immutable_world = world.clone();
    let mutable_world = world;

    for (ball_transform, ball, sprite) in ball_query.iter_mut(mutable_world){
        ball_transform.position += ball.velocity;
        if ball_transform.position.y >= ((window.inner_size().height as i32)/2) as f32 ||  // TODO: add method for getting window edges
           ball_transform.position.y <= -((window.inner_size().height as i32)/2) as f32
        {
            ball.velocity.y *= -1.0;
        }
        
        if ball_transform.position.x >= ((window.inner_size().width as i32)/2) as f32|| // TODO: add method for getting window edges
           ball_transform.position.x <= -((window.inner_size().width as i32)/2) as f32
        {
            panic!("you lose") // joke
        }
        for (paddle_transform, paddle) in right_paddle.iter(&immutable_world){ // Iterating over right paddle components
            if crate::check_aabb(
                crate::ball::BALL_AABB,  // Check collision of ball aabb and paddle aabb
                ball_transform, 
                PADDLE_AABB, 
                paddle_transform
            ){
                ball.velocity.x *= -1.0; // It collided with paddle!
            }
        }

        for (paddle_transform, paddle) in left_paddle.iter(&immutable_world){ // Iterating over left paddle components
            if crate::check_aabb(
                crate::ball::BALL_AABB, // Check collision of ball aabb and paddle aabb
                ball_transform, 
                PADDLE_AABB, 
                paddle_transform
            ){
                ball.velocity.x *= -1.0; // It collided with paddle
            }
        }

    }


}