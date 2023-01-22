
#[derive(Debug, Clone, Copy)]
pub struct StandardMaterial{
    pub color: StrayColor
}

impl StandardMaterial{
    pub fn new(color: StrayColor) -> Self{
        Self{color}
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct StrayColor{
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: f32
}
