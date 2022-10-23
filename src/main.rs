mod generators;
mod geometry;
mod intersect;

use std::time;
use tiny_skia::*;

fn main() {
    let mut g = generators::Generator::<generators::ShortLines>::new();
    let n = 1000;
    let lines: Vec<_> = (0..n).map(|_| g.next()).collect();
    let now = time::Instant::now();
    let intersections = intersect::report_all_intersections::<intersect::SweepLineIntersector>(&lines);
    let elapsed = now.elapsed();
    println!("#Intersections found: {} (took {:.2?})", intersections.len(), elapsed);

    // Display results in a small graphics window

    let mut red = Paint::default();
    red.set_color_rgba8(255, 0, 0, 255);
    red.anti_alias = true;

    let mut blue = Paint::default();
    blue.set_color_rgba8(0, 0, 255, 255);
    blue.anti_alias = true;

    let stroke = Stroke {
        width: 2.0,
        ..Default::default()
    };

    let scale = 900.0;

    let mut pixmap = Pixmap::new(1000, 1000).unwrap();
    pixmap.fill(Color::WHITE);
    for l in lines {
        let path = {
            let mut pb = PathBuilder::new();
            pb.move_to(l.a.x * scale, l.a.y * scale);
            pb.line_to(l.b.x * scale, l.b.y * scale);
            pb.finish().unwrap()
        };
        pixmap.stroke_path(&path, &red, &stroke, Transform::from_translate(50.0, 50.0), None);
    }

    for i in intersections {
        let path = PathBuilder::from_circle(i.point.x * scale, i.point.y * scale, 5.0).unwrap();
        pixmap.fill_path(&path, &blue, FillRule::Winding, Transform::from_translate(50.0, 50.0), None);
    }

    pixmap.save_png("output.png").unwrap();
}


/*

use rand::Rng;
use crate::Orientation::{Clockwise, Collinear, CounterClockwise};

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    x: f32,
    y: f32
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Line {
    a: Point,
    b: Point
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Intersection {
    l1: Line,
    l2: Line,
    p: Point
}

impl Point {
    // Check if p is on the given line segment, assuming that p, l.a and l.b are collinear
    pub fn on_segment(&self, l: Line) -> bool {
        self.x < f32::max(l.a.x, l.b.x) && self.x > f32::min(l.a.x, l.b.x)
            && self.y < f32::max(l.a.y, l.b.y) && self.y > f32::min(l.a.y, l.b.y)
    }
}

fn random_unit_square_line(rng: &mut impl Rng) -> Line {
    Line {
        a: Point{x: rng.gen::<f32>(), y: rng.gen::<f32>()},
        b: Point{x: rng.gen::<f32>(), y: rng.gen::<f32>()},
    }
}

#[derive(PartialEq)]
enum Orientation {
    Collinear,
    Clockwise,
    CounterClockwise
}

fn orientation(p: Point, q: Point, r: Point) -> Orientation {
    let v = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    let epsilon = 10e-3;
    if v.abs() < epsilon {
        return Collinear;
    }
    if v > 0.0 {
        return Clockwise;
    }
    CounterClockwise
}

fn intersect(l1: Line, l2: Line) -> bool {
    let o1 = orientation(l1.a, l2.a, l1.b);
    let o2 = orientation(l1.a, l2.a, l2.b);
    let o3 = orientation(l1.b, l2.b, l1.a);
    let o4 = orientation(l1.b, l2.b, l2.a);

    if o1 != o2 && o3 != o4 { return true; }

    if o1 == Collinear && l1.a.on_segment(Line { a: l1.b, b: l2.a }) { return true; }
    if o2 == Collinear && l1.a.on_segment(Line { a: l2.b, b: l2.a }) { return true; }
    if o3 == Collinear && l1.b.on_segment(Line { a: l1.a, b: l1.b }) { return true; }
    if o4 == Collinear && l1.b.on_segment(Line { a: l2.a, b: l2.b }) { return true; }

    false
}

fn find_all_intersections(lines: &[Line]) -> Vec<Intersection> {
    let mut intersections = Vec::<Intersection>::new();
    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            let l1 = lines[i];
            let l2 = lines[j];
            if intersect(l1, l2) {
                intersections.push(
                    Intersection {
                        l1, l2, p: Point{ x: 0.0, y: 0.0 }
                    }
                );
            }
        }
    }
    intersections
}

fn main() {
    let mut rng = rand::thread_rng();
    let n = 10;
    let mut lines: Vec<_> = (0..n).map(|_| random_unit_square_line(&mut rng)).collect();
    let intersect = find_all_intersections(&lines);
    println!("{:?}", intersect);
}

mod tests {
    use crate::*;

    #[test]
    fn test() {
        let l1 = Line { a: Point { x: 0.0, y: 0.0 }, b: Point { x: 1.0, y: 1.0 } };
        let l2 = Line { a: Point { x: 0.0, y: 1.0 }, b: Point { x: 1.0, y: 0.0 } };

        let intersections = find_all_intersections(&vec![l1, l2]);
        println!("{:?}", intersections);
        assert_eq!(intersections.len(), 1);
    }

}

*/
