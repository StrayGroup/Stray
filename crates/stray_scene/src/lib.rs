use cgmath::*;
use wgpu::{*, util::DeviceExt};
use image::*;

use stray_texture::StrayTexture;
use stray_material::*;

pub struct Sprite{
    pub texture: StrayTexture,
    pub local_position: Transform2D
}

impl Sprite{
    pub fn new(bytes: &[u8]) -> Self{
        let image = load_from_memory(bytes).unwrap();
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        Self { texture: StrayTexture::with(rgba, dimensions), local_position: Transform2D::new(0,0,0) }
    }
    pub fn to_render_object(&self, device: &Device, config: &SurfaceConfiguration, queue: &Queue, layout: &BindGroupLayout) -> RenderObject{
        let raw_size = [config.width as i32,config.height as i32];
        let vertices_data = vec![
            TextureVertex::new(-200, -200, 0.0, 1.0), TextureVertex::new(200, -200, 1.0, 1.0), TextureVertex::new(-200, 200, 0.0, 0.0), 
            TextureVertex::new(-200, 200, 0.0, 0.0), TextureVertex::new(200, -200, 1.0, 1.0 ), TextureVertex::new(200, 200, 1.0, 0.0),
        ];
        let vertices: Vec<RawVertex> = vertices_data.iter().map(|x| x.to_raw(raw_size)).collect();
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_len = vertices_data.len() as u32;
        let bind_group = self.texture.write_texture(device, queue, layout);
        RenderObject { 
            type_id: 1, 
            vertex: Some(StrayVertexBuffer(Some(vertex_buffer), vertex_buffer_len)), 
            index: None, 
            bind_group: Some(bind_group) 
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transform2D{
    pub position: Vector2<i32>,
    pub rotation: Deg<i32>
}

impl Transform2D{
    pub fn new(x: i32, y: i32, rotation: i32) -> Self {
        Self { position: Vector2::new(x, y), rotation: Deg(rotation)}
    }
}

// Wgpu indices buffer wrapper for resource need
pub struct StrayIndexBuffer(pub Option<Buffer>,pub u32);  

// Wgpu vertex buffer wrapper for resource need
pub struct StrayVertexBuffer(pub Option<Buffer>, pub u32);

// Wgpu render pipline wrapper for texture rendering
pub struct StrayTextureRenderPipeline(pub RenderPipeline, pub BindGroupLayout);

// Wgpu render pipeline wrapper for shape drawing
pub struct StrayShapeRenderPipeline(pub RenderPipeline);

pub struct RenderObject{
    // 0 is normal drawing, 1 is texture drawing
    pub type_id: i32,
    pub vertex: Option<StrayVertexBuffer>,
    pub index: Option<StrayIndexBuffer>,
    pub bind_group: Option<BindGroup>
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

pub struct RenderQuery(pub Vec<RenderObject>);

pub struct EngineData<D>(pub D);

impl <D>EngineData<D>{
    pub fn set(&mut self, data: D){
        self.0 = data;
    }
}



#[derive(Debug, Copy, Clone)]
pub struct TextureVertex{
    x: i32,
    y: i32,
    layer: i16,
    tex_coordx: f32,
    tex_coordy: f32,
    pub material: StandardMaterial,
}

impl TextureVertex{
    pub fn new(x: i32, y: i32, tex_coordx: f32, tex_coordy: f32) -> Self{
        Self{x: x, y: y, layer: 0, tex_coordx: tex_coordx, tex_coordy: tex_coordy, material: StandardMaterial::new(StrayColor::default())}
    }

    pub fn to_raw(&self, win_size: [i32;2]) -> RawVertex{
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
            ]


            }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex{
    x: i32,
    y: i32,
    layer: i16,
    pub material: StandardMaterial,
}

impl Vertex{
    pub fn new(x: i32, y: i32) -> Self{
        Self{x: x, y: y, layer: 0, material: StandardMaterial::new(StrayColor::default())}
    }

    pub fn to_raw(&self, win_size: [i32;2]) -> RawVertex{
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
            ]


            }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawVertex{
    position: [f32;3],
    tex_coords: [f32;2],
    color: [f32;4],
}

impl RawVertex{
    const ATTRIBS: [VertexAttribute; 3] =
        vertex_attr_array![0 => Float32x3, 1 => Float32x2, 2 => Float32x4];

    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
