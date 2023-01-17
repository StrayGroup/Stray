use stray_material::StandardMaterial;
use stray_material::Color;
use wgpu::*;
use std::mem;

pub struct Draw{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: Option<StandardMaterial>,
}

impl Draw{
    pub fn init() -> Self{
        Self{vertices: vec![], indices: vec![], material: None}
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex> ){
        self.vertices = vertices;
    }

    pub fn set_indices(&mut self, indices: Vec<u16>){
        self.indices = indices;
    }

    pub fn set_material(&mut self, material: StandardMaterial){
        self.material = Some(material);
        for v in self.vertices.iter_mut(){
            v.material = material;
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct Vertex{
    x: i32,
    y: i32,
    layer: i16,
    material: StandardMaterial,
}


impl Vertex{
    pub fn new(x: i32, y: i32) -> Self{
        Self{x: x, y: y, layer: 0, material: StandardMaterial::new(Color::default())}
    }

    pub fn to_raw(&self, win_size: [i32;2]) -> RawVertex{
        RawVertex { 
            position: [
                self.x as f32/win_size[0] as f32, 
                self.y as f32/win_size[1] as f32, 
                self.layer as f32], 
            color: [
                (((self.material.color.r / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.g / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                (((self.material.color.b / 255) as f32 + 0.055) / 1.055).powf(2.4), 
                self.material.color.a]
            }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVertex{
    position: [f32;3],
    color: [f32;4],
}

impl RawVertex{
    const ATTRIBS: [VertexAttribute; 2] =
        vertex_attr_array![0 => Float32x3, 1 => Float32x4];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}