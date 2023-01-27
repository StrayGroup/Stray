use glam::vec2;
use wgpu::{*, util::DeviceExt};

use image::*;

use stray_texture::StrayTexture;
use stray_material::StandardMaterial;

use crate::engine_data::{
    TextureVertex,
    RawVertex,
    RenderObject,
    StrayVertexBuffer,
    StrayIndexBuffer,
    Vertex
};
use crate::types::Transform2D;

pub struct Sprite{
    pub texture: StrayTexture,
    pub layer: i32,
}

impl Sprite{
    pub fn new(bytes: &[u8], layer: i32) -> Self{
        let image = load_from_memory(bytes).unwrap();
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        Self { texture: StrayTexture::with(rgba, dimensions) , layer: layer}
    }
    pub fn to_render_object(&self, 
        device: &Device, 
        config: &SurfaceConfiguration, 
        queue: &Queue, 
        layout: &BindGroupLayout,
        transform: &Transform2D
    ) -> RenderObject{
        let th = (self.texture.dimensions.1 as f32 * transform.scale) as i32 ;
        let tw = (self.texture.dimensions.0 as f32 * transform.scale) as i32;
        let raw_size = [config.width as i32,config.height as i32];
        let vertices_data = vec![
            TextureVertex::new(-tw, -th, self.layer, 0.0, 1.0), 
            TextureVertex::new(tw, -th, self.layer, 1.0, 1.0), 
            TextureVertex::new(-tw, th, self.layer, 0.0, 0.0), 
            TextureVertex::new(-tw, th, self.layer, 0.0, 0.0), 
            TextureVertex::new(tw, -th, self.layer, 1.0, 1.0 ), 
            TextureVertex::new(tw, th, self.layer, 1.0, 0.0),
        ];

        // Apply transform scale and make position in [-1,1] range
        let true_transform = Transform2D { 
            position: transform.position/vec2((config.width/2) as f32, (config.height/2) as f32), 
            rotation: transform.rotation, 
            scale: transform.scale };
        
        let vertices: Vec<RawVertex> = vertices_data.iter().map(|x| x.to_raw(raw_size, &true_transform)).collect();
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_len = vertices_data.len() as u32;
        let bind_group = self.texture.write_texture(device, queue, layout);
        RenderObject { 
            type_id: 1, 
            vertex: Some(StrayVertexBuffer(Some(vertex_buffer), vertex_buffer_len)), 
            index: None, 
            bind_group: Some(bind_group),
        }
    }
}


pub struct ScreenDraw{
    pub transform: Transform2D,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: Option<StandardMaterial>,
}

impl ScreenDraw{
    pub fn init(x: i32, y: i32, rotation: i32) -> Self{
        Self{transform: Transform2D::new(x, y, rotation, 1.0),vertices: vec![], indices: vec![], material: None}
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

    pub fn create_indices_buffer(&self, device: &Device) -> StrayIndexBuffer{
        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Indices Buffer"),
                contents: bytemuck::cast_slice(self.indices.as_slice()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let index_buffer_len = self.indices.len() as u32;
        StrayIndexBuffer(Some(index_buffer), index_buffer_len)
    }

    pub fn create_vertex_buffer(&self, device: &Device, config: &SurfaceConfiguration) -> StrayVertexBuffer{
        let raw_size = [config.width as i32,config.height as i32];
        let true_transform = Transform2D { 
            position: self.transform.position/vec2((config.width/2) as f32, (config.height/2) as f32), 
            rotation: self.transform.rotation, 
            scale: self.transform.scale };
        let vertices: Vec<RawVertex> = self.vertices.iter().map(|x| x.to_raw(raw_size, true_transform)).collect();
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices.as_slice()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let vertex_buffer_len = self.vertices.len() as u32;
        StrayVertexBuffer(Some(vertex_buffer), vertex_buffer_len)
    }
    pub fn to_render_object(&self, device: &Device, config: &SurfaceConfiguration) -> RenderObject{
        let vertex = self.create_vertex_buffer(&device, &config);
        let index = self.create_indices_buffer(&device);
        RenderObject{
            type_id: 0,
            vertex: Some(vertex),
            index: Some(index),
            bind_group: None
        }
    }
}