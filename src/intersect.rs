use crate::geometry;
use std::ops;
use std::collections::HashSet;
use crate::geometry::Line;

#[derive(Default, Debug, Clone, Copy)]
pub struct Intersection {
    pub l1: geometry::Line,
    pub l2: geometry::Line,
    pub point: geometry::Point,
}

#[derive(Default)]
pub struct Report {
    pub intersections: Vec<Intersection>,
    pub num_tests: usize
}

pub trait Intersector {
    fn report_intersections(lines: &[geometry::Line]) -> Report;
}

pub fn report_all_intersections<I>(lines: &[geometry::Line]) -> Report where I: Intersector {
    I::report_intersections(&lines)
}

impl ops::Add<geometry::Point> for geometry::Point {
    type Output = geometry::Point;

    fn add(self, rhs: geometry::Point) -> Self::Output {
        geometry::Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<geometry::Point> for geometry::Point {
    type Output = geometry::Point;

    fn sub(self, rhs: geometry::Point) -> Self::Output {
        geometry::Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Mul<f32> for geometry::Point {
    type Output = geometry::Point;

    fn mul(self, rhs: f32) -> Self::Output {
        geometry::Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// scalar-point multiplication is commutative
impl ops::Mul<geometry::Point> for f32 {
    type Output = geometry::Point;

    fn mul(self, rhs: geometry::Point) -> Self::Output {
        rhs * self
    }
}

fn cross2d(v: geometry::Point, w: geometry::Point) -> f32 {
    v.x * w.y - v.y * w.x
}

// https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect
fn line_intersect(l1: geometry::Line, l2: geometry::Line) -> Option<Intersection> {
    // Line 1 goes from p to p + r
    // Line 2 goes from q to q + s
    let p = l1.a;
    let r = l1.b - p;
    let q = l2.a;
    let s = l2.b - q;

    // Now, line 1 and line 2 intersect if there is a t and u such that
    // p + t * r = q + u * s

    // There are four cases:
    // 1) r x s = 0 and (q - p) x r = 0 ==> Collinear. Our intersector will not consider collinear lines.
    // 2) r x s = 0 and (q - p) x r != 0 ==> Parallel and non-intersecting.
    // 3) r x s != 0 and 0 <= t <= 1 and 0 <= u <= 1 ==> Intersection at point p + t * r (= q + u * s).
    // 4) Otherwise, the line segments are not parallel but do not intersect.

    let rs = cross2d(r, s);
    let qp_r = cross2d(q - p, r);
    let epsilon = 10e-3;

    // 1) Collinear
    if rs.abs() < epsilon && qp_r.abs() < epsilon {
        return None;
    }

    // 2) Parallel and non-intersecting
    if rs.abs() < epsilon && qp_r.abs() > epsilon {
        return None;
    }

    let t = cross2d(q - p, s) / rs;
    let u = qp_r / rs;

    // 3) Intersection
    if rs.abs() > epsilon && epsilon < t && t < 1.0f32 && epsilon < u && u < 1.0f32 {
        return Some(Intersection {
            l1,
            l2,
            point: p + t * r,
        });
    }

    // 4) No intersection
    None
}

pub struct BruteForceIntersector;
pub struct SweepLineIntersector;
pub struct SmartSweepLineIntersector;

impl Intersector for BruteForceIntersector {
    fn report_intersections(lines: &[Line]) -> Report {
        let mut intersections = Vec::<Intersection>::new();
        let mut num_tests = 0;
        for i in 0..lines.len()-1 {
            for j in i+1..lines.len() {
                if let Some(intersection) = line_intersect(lines[i], lines[j]) {
                    intersections.push(intersection);
                }
            }
        }
        Report { intersections, num_tests }
    }
}

#[derive(Default, Copy, Clone)]
struct SweepLinePoint {
    pub line: geometry::Line,
    pub point: geometry::Point,
    pub start: bool,
}

impl Intersector for SweepLineIntersector {
    fn report_intersections(lines: &[Line]) -> Report {
        let mut intersections = Vec::new();
        let mut num_tests = 0;

        let mut points = lines.iter().flat_map(|line| [
            // The largest y coordinate is considered the start of a line.
            SweepLinePoint { line: *line, point: line.a, start: line.a.y > line.b.y },
            SweepLinePoint { line: *line, point: line.b, start: line.b.y > line.a.y }])
        .collect::<Vec<_>>();
        points.sort_by(|p, q| p.point.y.partial_cmp(&q.point.y).unwrap().reverse());
        let mut active_set = Vec::new();

        for current_sweep_point in 0..points.len() {
            let point = points[current_sweep_point];
            if point.start {
                for other in &active_set {
                    num_tests += 1;
                    if let Some(intersect) = line_intersect(point.line, *other) {
                        intersections.push(intersect);
                    }
                }
                active_set.push(point.line);
            } else {
                active_set.retain(|x| *x != point.line);
            }
        }

        Report { intersections, num_tests }
    }
}

impl Intersector for SmartSweepLineIntersector {
    fn report_intersections(lines: &[Line]) -> Report {
        let mut intersections = Vec::new();
        let mut num_tests = 0;



        Report { intersections, num_tests }
    }
}