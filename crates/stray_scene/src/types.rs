use std::f32::consts::PI;

use glam::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform2D{
    pub position: Vec2,
    pub rotation: i32,
    pub scale: f32,
}

impl Transform2D{
    pub const ZERO: Self = Self{
        position: vec2(0.0,0.0),
        rotation: 0,
        scale: 1.0
    };

    pub const fn new(x: i32, y: i32, rotation: i32,scale: f32) -> Self {
        Self { position: Vec2::new(x as f32, y as f32), rotation: rotation, scale: scale}
    }

    pub fn to_raw(&self) -> [[f32;4];4]{
        let rotation: Quat = Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, deg2rad(self.rotation as f32));
        (Mat4::from_translation(
            vec3(
                self.position.x, 
                self.position.y, 
                0.0
            )
        ) * Mat4::from_quat(rotation)).to_cols_array_2d()
    }
}


pub fn deg2rad(deg: f32) -> f32{
    deg * (PI/180.0)
}

pub fn rad2deg(rad: f32) -> f32{
    rad / (PI*180.0)
}