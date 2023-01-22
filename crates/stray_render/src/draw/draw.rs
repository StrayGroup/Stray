use wgpu::{*, util::DeviceExt};

use stray_material::{
    StandardMaterial, 
};

use stray_scene::{
    StrayIndexBuffer,
    StrayVertexBuffer,
    RenderObject,
    Vertex,
    RawVertex
};

pub struct ScreenDraw{
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: Option<StandardMaterial>,
}

impl ScreenDraw{
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
        let vertices: Vec<RawVertex> = self.vertices.iter().map(|x| x.to_raw(raw_size)).collect();
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



