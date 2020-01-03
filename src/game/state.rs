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

pub struct Ground {
    pub boxes: Vec<Vec2>,
    pub dirty: DirtyFlags
}

pub struct Cat {
    pub direction: Option<f32>,
    pub flying: bool,
    pub path: VecDeque<Vec2>,
    pub tail: VecDeque<Vec2>
}

pub struct State {
    pub input: Input,
    pub ground: Ground,
    pub cat: Cat
}
