use std::sync::atomic::{AtomicUsize, Ordering};
use crate::geometry;
use rand::Rng;

pub struct Generator<F> {
    gen: F,
    cur: i32,
}

pub trait LineGenerator {
    fn line(&mut self, i: i32) -> geometry::Line;
}

impl<F> Generator<F> where F: LineGenerator + Default {
    pub fn new() -> Generator<F> {
        Generator {
            gen: F::default(),
            cur: 0
        }
    }

    pub fn next(&mut self) -> geometry::Line {
        let i = self.cur;
        self.cur += 1;
        self.gen.line(i)
    }
}

fn random_unit_point(rng: &mut impl Rng) -> geometry::Point {
    geometry::Point {
        x: rng.gen::<f32>(),
        y: rng.gen::<f32>()
    }
}

fn random_point_in_circle(center: geometry::Point, radius: f32, rng: &mut impl Rng) -> geometry::Point {
    // Generate random point around circle at origin in spherical coordinates, then convert
    // to cartesian and move to be centered around the given center point.
    let r = rng.gen_range(0f32..radius);
    let theta = rng.gen_range(0f32..2f32 * std::f32::consts::PI);
    // Convert spherical to cartesian
    let x = r * theta.cos();
    let y = r * theta.sin();
    geometry::Point {
        x: x + center.x,
        y: y + center.y
    }
}

static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
fn line_id() -> usize {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Default)]
pub struct RandomUnitSquare {
    rng: rand::rngs::ThreadRng,
}

impl LineGenerator for RandomUnitSquare {
    fn line(&mut self, _: i32) -> geometry::Line {
        geometry::Line {
            id: line_id(), a: random_unit_point(&mut self.rng), b: random_unit_point(&mut self.rng)
        }
    }
}

#[derive(Default)]
pub struct ShortLines {
    rng: rand::rngs::ThreadRng,
}

impl LineGenerator for ShortLines {
    fn line(&mut self, _: i32) -> geometry::Line {
        let start = random_unit_point(&mut self.rng);
        let length = self.rng.gen_range(0f32..0.25);
        let end = random_point_in_circle(start, length, &mut self.rng);
        geometry::Line { id: line_id(), a: start, b: end }
    }
}