use crate::geometry;
use rand::Rng;

pub trait LineGenerator {
    fn line(rng: &mut impl Rng) -> geometry::Line;
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

pub struct RandomUnitSquare;
pub struct ShortLines;

pub fn generate_lines<G>(n: usize, rng: &mut impl Rng) -> Vec<geometry::Line> where G : LineGenerator {
    (0..n).into_iter().map(|_|
        G::line(rng)
    )
    .collect()
}

impl LineGenerator for RandomUnitSquare {
    fn line(rng: &mut impl Rng) -> geometry::Line {
        geometry::Line {
            a: random_unit_point(rng), b: random_unit_point(rng)
        }
    }
}

impl LineGenerator for ShortLines {
    fn line(rng: &mut impl Rng) -> geometry::Line {
        let start = random_unit_point(rng);
        let length = rng.gen_range(0f32..0.25);
        let end = random_point_in_circle(start, length, rng);
        geometry::Line { a: start, b: end }
    }
}