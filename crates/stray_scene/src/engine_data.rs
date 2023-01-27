use wgpu::*;

use stray_material::{
    StandardMaterial,
    StrayColor,
};

use crate::types::Transform2D;


pub struct RenderObject{
    // 0 is normal drawing, 1 is texture drawing
    pub type_id: i32,
    pub vertex: Option<StrayVertexBuffer>,
    pub index: Option<StrayIndexBuffer>,
    pub bind_group: Option<BindGroup>,
}

impl RenderObject{
    pub fn get_type(&self) -> i32{
        self.type_id
    }
    pub fn get_vertex(&self) -> &StrayVertexBuffer{
        self.vertex.as_ref().unwrap()
    }
    pub fn get_index(&self) -> &StrayIndexBuffer{
        self.index.as_ref().unwrap()
    }
    pub fn get_bind_group(&self) -> &BindGroup{
        self.bind_group.as_ref().unwrap()
    }
}


#[derive(Debug, Copy, Clone)]
pub struct TextureVertex{
    x: i32,
    y: i32,
    layer: i32,
    tex_coordx: f32,
    tex_coordy: f32,
    pub material: StandardMaterial,
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


pub struct StrayIndexBuffer(pub Option<Buffer>,pub u32);  
pub struct StrayVertexBuffer(pub Option<Buffer>, pub u32);
pub struct StrayTextureRenderPipeline(pub RenderPipeline, pub BindGroupLayout);
pub struct StrayShapeRenderPipeline(pub RenderPipeline);
pub struct RenderQuery(pub Vec<RenderObject>);
pub struct EngineData<D>(pub D);

impl <D>EngineData<D>{
    pub fn set(&mut self, data: D){
        self.0 = data;
    }
}
