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
    pub forward: bool
}

pub struct Ground {
    pub boxes: Vec<Vec2>,
    pub dirty: DirtyFlags
}

#[derive(PartialEq)]
pub enum CatMovement {
    None,
    Forward,
    Left,
    Right
}

pub struct Cat {
    pub movement: CatMovement,
    pub path: VecDeque<Vec2>,
    pub tail: VecDeque<Vec2>
}

pub struct State {
    pub input: Input,
    pub ground: Ground,
    pub cat: Cat
}
