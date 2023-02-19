use glam::{vec2, Vec2};
use wgpu::{*, util::DeviceExt};

use image::*;
use hal::*;

use stray_texture::StrayTexture;
use stray_material::StandardMaterial;

use crate::engine_data::{
    TextureVertex,
    RawVertex,
    SVertexBuffer,
    SIndexBuffer,
    Vertex
};
use crate::types::Transform2D;

pub struct Sprite{
    pub id: u32,
    pub texture: StrayTexture,
    pub layer: i32,
}



impl Sprite{
    pub fn new(id: u32, bytes: &[u8], layer: i32) -> Self{
        let image = load_from_memory(bytes).unwrap();
        let rgba = image.to_rgba8();
        let dimensions = image.dimensions();
        Self { id: id, texture: StrayTexture::with(rgba, dimensions) , layer: layer}
    }

}

#[derive(Debug, Clone)]
pub struct Canvas{
    pub id: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub material: Option<StandardMaterial>,
}

impl Canvas{
    pub fn init(id: u32) -> Self{
        Self{id: id, vertices: vec![], indices: vec![], material: None}
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D{
    pub position: Vec2,
    pub rotation: f32,
    pub scale: f32
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2D{
    points: [Point2D;2]
}

impl Line2D{
    pub fn new(p1: Point2D, p2: Point2D) -> Self{
        Self{
            points: [p1,p2]
        }
    }
    pub fn right(&mut self) -> Point2D{
        todo!()
    }
    pub fn left(&mut self) -> Point2D{
        todo!()
    }
    pub fn top(&mut self) -> Point2D{
        todo!()
    }
    pub fn bottom(&mut self) -> Point2D{
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiLine2D{
    points: Vec<Point2D>
}

impl MultiLine2D{
    pub fn new(points: Vec<Point2D>) -> Self{
        Self{
            points
        }
    }
}