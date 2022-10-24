use std::hash::{Hash, Hasher};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub a: Point,
    pub b: Point
}