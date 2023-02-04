use winit::{
    event::*,
    dpi::PhysicalPosition
};

// WIP
pub enum MouseEvent{
    ENTERED,
    MOVED(PhysicalPosition<i64>)
}

pub type Key = VirtualKeyCode;

pub enum LastState{
    PRESSED(Key),
    RELEASED(Key),
    NONE
}
pub enum InputEvent{
    PRESSED(Key),
    RELEASED(Key),
    NONE
}

impl InputEvent{
    pub fn is_pressed(&self, vk: Key) -> bool{
        match self{
            Self::PRESSED(key) if key == &vk =>  true,
            _ => false
        }
    }

    pub fn is_released(&self, vk: Key) -> bool{
        match self{
            Self::RELEASED(key) if key == &vk =>  true,
            _ => false
        }
    }
}

impl From<&KeyboardInput> for InputEvent{
    fn from(value: &KeyboardInput) -> Self {
        match value.state{
            ElementState::Pressed => Self::PRESSED(value.virtual_keycode.unwrap()),
            ElementState::Released => Self::RELEASED(value.virtual_keycode.unwrap())
        }
    }
}


impl LastState{
    pub fn was_pressed(&self, vk: Key) -> bool{
        match self{
            Self::PRESSED(key) if key == &vk =>  true,
            _ => false
        }
    }

    pub fn was_released(&self, vk: Key) -> bool{
        match self{
            Self::RELEASED(key) if key == &vk =>  true,
            _ => false
        }
    }
}

impl From<&KeyboardInput> for LastState{
    fn from(value: &KeyboardInput) -> Self {
        match value.state{
            ElementState::Pressed => Self::PRESSED(value.virtual_keycode.unwrap()),
            ElementState::Released => Self::RELEASED(value.virtual_keycode.unwrap())
        }
    }
}