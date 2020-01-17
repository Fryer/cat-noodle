use std::collections::VecDeque;

use lib::math::Vec2;


bitflags! {
    pub struct DirtyFlags: u8 {
        const NONE = 0;
        const RENDER = 0b1;
        const PHYSICS = 0b10;
        const ALL = Self::RENDER.bits | Self::PHYSICS.bits;
    }
}

pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub fly: bool
}

pub enum DebugShape {
    Line(f32, f32, f32, f32),
    Circle(f32, f32, f32)
}

pub struct DebugColor(pub u8, pub u8, pub u8, pub u8);

pub struct DebugInfo {
    pub shapes: VecDeque<(DebugShape, DebugColor)>
}

pub struct Ground {
    pub boxes: Vec<Vec2>,
    pub dirty: DirtyFlags
}

pub struct Cat {
    pub direction: Option<f32>,
    pub flying: bool,
    pub path: VecDeque<Vec2>,
    pub tail: VecDeque<Vec2>,
    pub grab_d: Option<Vec2>
}

pub struct State {
    pub input: Input,
    pub debug: DebugInfo,
    pub ground: Ground,
    pub cat: Cat
}
