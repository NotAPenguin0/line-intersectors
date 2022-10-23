use std::hash::{Hash, Hasher};

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Line {
    pub id: usize, // unique ID for each line, used to hash
    pub a: Point,
    pub b: Point
}

impl PartialEq<Self> for Line {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Line {}

impl Hash for Line {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.id);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) where Self: Sized {
        data.iter().for_each( |line| line.hash(state));
    }
}