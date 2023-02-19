use wgpu::*;
use hal::*;

use stray_material::{
    StandardMaterial,
    StrayColor,
};


use crate::types::Transform2D;


#[derive(Debug, Copy, Clone)]
pub struct TextureVertex{
    x: i32,
    y: i32,
    layer: i32,
    tex_coordx: f32,
    tex_coordy: f32,
    material: StandardMaterial,
}

impl TextureVertex{
    pub fn new(x: i32, y: i32, layer: i32, tex_coordx: f32, tex_coordy: f32) -> Self{
        Self{x: x, y: y, layer: layer, tex_coordx: tex_coordx, tex_coordy: tex_coordy, material: StandardMaterial::new(StrayColor::default())}
    }

    pub fn to_raw(&self, win_size: [i32;2], transform: &Transform2D) -> RawVertex{
        RawVertex { 
            position: [
                self.x as f32/win_size[0] as f32, 
                self.y as f32/win_size[1] as f32, 
                self.layer as f32
            ], 
            tex_coords: [
                self.tex_coordx, self.tex_coordy
            ],
            color: [
                (((self.material.color.r / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.g / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.b / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                self.material.color.a
            ],
            model: transform.to_raw()


            }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex{
    pub x: i32,
    pub y: i32,
    layer: i32,
    
    pub material: StandardMaterial,
}

impl Vertex{
    pub fn new(x: i32, y: i32, layer: i32) -> Self{
        Self{x: x, y: y, layer: layer, material: StandardMaterial::new(StrayColor::default())}
    }

    pub fn to_raw(&self, win_size: [i32;2], transform: Transform2D) -> RawVertex{
        RawVertex { 
            position: [
                self.x as f32/win_size[0] as f32, 
                self.y as f32/win_size[1] as f32, 
                self.layer as f32
            ], 
            tex_coords: [
                0.,0.
            ],
            color: [
                (((self.material.color.r / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.g / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.b / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                self.material.color.a
            ],
            model: transform.to_raw()


            }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVertex{
    position: [f32;3],
    tex_coords: [f32;2],
    color: [f32;4],
    model: [[f32;4];4]
}

impl RawVertex{
    const ATTRIBS: [VertexAttribute; 7] =
        vertex_attr_array![
            0 => Float32x3, 
            1 => Float32x2, 
            2 => Float32x4, 
            // Matrix
            3 => Float32x4,
            4 => Float32x4,
            5 => Float32x4,
            6 => Float32x4
        ];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}


pub struct SIndexBuffer(pub Option<Buffer>,pub u32);  
pub struct SVertexBuffer(pub Option<Buffer>, pub u32);

impl SIndexBuffer{
    pub fn with(data: (Buffer, u32)) -> Self{
        Self(Some(data.0), data.1)
    }

    pub fn buffer(&self) -> &Option<Buffer>{
        &self.0
    }

    pub fn len(&self) -> u32{
        self.1
    }
}

impl SVertexBuffer{
    pub fn with(data: (Buffer, u32)) -> Self{
        Self(Some(data.0), data.1)
    }
    
    pub fn buffer(&self) -> &Option<Buffer>{
        &self.0
    }

    pub fn len(&self) -> u32{
        self.1
    }
}