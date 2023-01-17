
#[derive(Debug, Clone, Copy)]
pub struct StandardMaterial{
    pub color: Color
}

impl StandardMaterial{
    pub fn new(color: Color) -> Self{
        Self{color}
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Color{
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: f32
}
