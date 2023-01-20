use cgmath::*;

#[derive(Debug, Clone, Copy)]

pub struct Sprite{
    pub texture_id: i32,
    pub local_position: Transform2D
}

impl Sprite{
    pub fn new(texture_id: i32) -> Self{
        Self { texture_id: texture_id, local_position: Transform2D::new(0,0,0) }
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
pub struct Container<T>(pub Vec<T>);

pub struct EngineData<D>(pub D);

impl <D>EngineData<D>{
    pub fn set(&mut self, data: D){
        self.0 = data;
    }
}
