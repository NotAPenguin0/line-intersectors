use crate::geometry;
use std::ops;
use std::collections::HashSet;

#[derive(Default, Debug, Clone, Copy)]
pub struct Intersection {
    pub l1: geometry::Line,
    pub l2: geometry::Line,
    pub point: geometry::Point,
}

pub trait Intersector {
    fn new(lines: &[geometry::Line]) -> Self;
    fn num_tests(&mut self) -> usize;
    fn next_intersection(&mut self) -> Option<Intersection>;
}

pub fn report_all_intersections<I>(lines: &[geometry::Line]) -> Vec<Intersection> where I: Intersector {
    let mut intersections = Vec::new();
    let mut intersector = I::new(&lines);
    while let Some(intersection) = intersector.next_intersection() {
        intersections.push(intersection);
    }
    println!("#intersection tests: {:?}", intersector.num_tests());
    intersections
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

#[derive(Default)]
pub struct BruteForceIntersector {
    lines: Vec<geometry::Line>,
    // Indices for iteration over the list of lines.
    i: usize,
    j: usize,
    pub num_tests: usize
}

impl Intersector for BruteForceIntersector {
    fn new(lines: &[geometry::Line]) -> BruteForceIntersector {
        BruteForceIntersector {
            lines: Vec::from(lines),
            i: 0,
            j: 1,
            num_tests: 0
        }
    }

    fn num_tests(&mut self) -> usize {
        self.num_tests
    }

    fn next_intersection(&mut self) -> Option<Intersection> {
        while self.i < self.lines.len() - 1 {
            while self.j < self.lines.len() {
                let line = self.lines[self.i];
                let other = self.lines[self.j];
                self.j += 1;
                self.num_tests += 1;
                if let Some(intersection) = line_intersect(line, other) {
                    return Some(intersection);
                }
            }
            self.i += 1;
            self.j = self.i + 1;
        }
        None
    }
}

#[derive(Default)]
pub struct SweepLineIntersector {
    lines: Vec<geometry::Line>,
    points: Vec<SweepLinePoint>,
    current_sweep_point: usize,
    active_set: HashSet<geometry::Line>,
    to_report: Vec<Intersection>,

    // Performance analytics
    pub num_tests: usize
}

#[derive(Default, Copy, Clone)]
struct SweepLinePoint {
    pub line: geometry::Line,
    pub point: geometry::Point,
    pub start: bool,
}

impl Intersector for SweepLineIntersector {
    fn new(lines: &[geometry::Line]) -> SweepLineIntersector {
        let mut intersector = SweepLineIntersector {
            lines: Vec::from(lines),
            points: lines.iter().flat_map(|line| [
                    // The largest y coordinate is considered the start of a line.
                    SweepLinePoint { line: *line, point: line.a, start: line.a.y > line.b.y },
                    SweepLinePoint { line: *line, point: line.b, start: line.b.y > line.a.y }])
                .collect::<Vec<_>>(),
            current_sweep_point: 0,
            active_set: HashSet::new(),
            ..Default::default()
        };
        intersector.points.sort_by(|p, q| p.point.y.partial_cmp(&q.point.y).unwrap().reverse());
        intersector
    }

    fn num_tests(&mut self) -> usize {
        self.num_tests
    }

    fn next_intersection(&mut self) -> Option<Intersection> {
        while self.current_sweep_point < self.points.len() {
            let p = self.points[self.current_sweep_point];

            // Start event: add to active set and test against all current elements in the active set.
            if p.start {
                for other in &self.active_set {
                    self.num_tests += 1;
                    if let Some(intersect) = line_intersect(p.line, *other) {
                        self.to_report.push(intersect);
                    }
                }
                self.active_set.insert(p.line);
            } else {
                // End event: remove from active set
                self.active_set.remove(&p.line);
            }

            self.current_sweep_point += 1;
        }

        // If we still have intersections to report, report them and remove from the list
        if !self.to_report.is_empty() {
            return Some(self.to_report.swap_remove(0));
        }

        None
    }
}