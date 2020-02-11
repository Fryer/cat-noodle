use std::collections::VecDeque;
use std::time;

use lib::math::Vec2;


bitflags! {
    pub struct DirtyFlags: u8 {
        const RENDER = 0b1;
        const PHYSICS = 0b10;
    }
}

pub struct Input {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub turn: bool,
    pub extend: bool,
    pub contract: bool,
    pub fly: bool,
    pub toggle_pause: bool,
    pub step: bool,
    pub toggle_debug_physics: bool,
    pub toggle_debug_physics_shapes: bool,
    pub toggle_debug_physics_joints: bool,
    pub toggle_debug_physics_aabbs: bool,
    pub toggle_debug_physics_transforms: bool,
    pub toggle_debug_physics_contacts: bool
}

pub enum DebugShape {
    Line(f32, f32, f32, f32),
    Circle(f32, f32, f32)
}

pub struct DebugColor(pub u8, pub u8, pub u8, pub u8);

bitflags! {
    pub struct DebugPhysics: u8 {
        const SHAPES = 0b1;
        const JOINTS = 0b10;
        const AABBS = 0b100;
        const TRANSFORMS = 0b1000;
        const CONTACTS = 0b10000;
    }
}

pub struct DebugInfo {
    pub shapes: VecDeque<(DebugShape, DebugColor)>,
    pub frames: VecDeque<time::Instant>,
    pub skipped_steps: bool,
    pub paused: bool,
    pub show_physics: bool,
    pub physics_flags: DebugPhysics
}

pub struct Ground {
    pub boxes: Vec<Vec2>,
    pub dirty: DirtyFlags
}

pub struct Cat {
    pub direction: Option<f32>,
    pub turning: bool,
    pub extending: bool,
    pub contracting: bool,
    pub flying: bool,
    pub path: VecDeque<Vec2>,
    pub tail: VecDeque<Vec2>,
    pub grab_d: Option<Vec2>,
    pub walk_phase: f32
}

pub struct State {
    pub paused: bool,
    pub input: Input,
    pub debug: DebugInfo,
    pub ground: Ground,
    pub cat: Cat
}
