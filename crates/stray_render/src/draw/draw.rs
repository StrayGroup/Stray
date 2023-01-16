use stray_material::StandardMaterial;
use stray_material::Color;

pub struct Draw{
    pub vertices: Vec<Vertex>,
    pub material: Option<StandardMaterial>,
}

impl Draw{
    pub fn init() -> Self{
        Self{vertices: vec![], material: None}
    }

    pub fn set_vertices(&mut self, vertices: Vec<Vertex> ){
        self.vertices = vertices;
    }

    pub fn set_material(&mut self, material: StandardMaterial){
        self.material = Some(material);
    }
}


#[derive(Debug)]
pub struct Vertex{
    x: i64,
    y: i64,
    layer: i16,
    color: Color
}


impl Vertex{
    pub fn new(x: i64, y: i64) -> Self{
        Self{x: x, y: y, layer: 0, color: Color::default()}
    }
}
