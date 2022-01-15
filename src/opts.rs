use crate::vec::Vector;

#[derive(Default)]
pub struct Output {
    pub width: usize,
    pub height: usize,
    pub bits: u32,
}

#[derive(Default)]
pub struct Render {
    pub max_reflections: u32,
    pub threads: u32,
}

#[derive(Default)]
pub struct World {
    pub cam_pos: Vector,
}

#[derive(Default)]
pub struct Config {
    pub output: Output,
    pub render: Render,
    pub world: World,
}
