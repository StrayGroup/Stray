use std::f32::consts::PI;

use glam::*;


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform2D{
    pub position: Vec2,
    pub rotation: i32,
    pub size: Vec2,
}

impl Transform2D{
    pub const ZERO: Self = Self{
        position: vec2(0.0,0.0),
        rotation: 0,
        size: vec2(1.0,1.0)
    };

    pub const fn new(position: Vec2, rotation: i32, size: Vec2) -> Self {
        Self { position, rotation, size}
    }

    pub fn to_raw(&self) -> [[f32;4];4]{
        let rotation: Quat = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, deg2rad(self.rotation as f32));
        (Mat4::from_translation(
            vec3(
                self.position.x, 
                self.position.y, 
                0.0
            )
        ) * Mat4::from_quat(rotation)
          * Mat4::from_scale(
            vec3(
                self.size.x,
                self.size.y,
                1.0
            )
          )).to_cols_array_2d()
    }
}


pub fn deg2rad(deg: f32) -> f32{
    deg * (PI/180.0)
}

pub fn rad2deg(rad: f32) -> f32{
    rad / (PI*180.0)
}