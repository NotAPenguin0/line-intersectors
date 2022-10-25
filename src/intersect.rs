#![feature(map_first_last)]
use std::cmp::Ordering;
use crate::geometry;
use std::ops;
use std::collections::{Bound, BTreeSet};
use std::ops::{RangeBounds, RangeTo, RangeFrom};
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

#[derive(Default, Debug, Copy, Clone)]
enum EventType {
    #[default]
    Start,
    End,
    Intersection
}

#[derive(Default, Debug, Copy, Clone)]
struct SweepLinePoint {
    pub line: geometry::Line,
    pub point: geometry::Point,
    pub event: EventType
}

impl PartialEq for SweepLinePoint {
    fn eq(&self, other: &Self) -> bool {
        let eps: f32 = 10e-3;
        f32::abs(self.point.x - other.point.x) < eps && f32::abs(self.point.y - other.point.y) < eps
    }
}

impl Eq for SweepLinePoint {}

impl PartialOrd for SweepLinePoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.point.y.partial_cmp(&self.point.y)
    }
}

impl Ord for SweepLinePoint {
    fn cmp(&self, other: &Self) -> Ordering {
        other.point.y.total_cmp(&self.point.y)
    }
}

impl Intersector for SweepLineIntersector {
    fn report_intersections(lines: &[Line]) -> Report {
        let mut intersections = Vec::new();
        let mut num_tests = 0;

        let mut points = lines.iter().flat_map(|line| [
            // The largest y coordinate is considered the start of a line.
            SweepLinePoint { line: *line, point: line.a,
                event: match line.a.y > line.b.y {
                    true => EventType::Start,
                    _ => EventType::End
                }
            },
            SweepLinePoint {
                line: *line,
                point: line.b,
                event: match line.b.y > line.a.y {
                    true => EventType::Start,
                    _ => EventType::End
                }
            }])
        .collect::<Vec<_>>();
        points.sort_by(|p, q| p.point.y.partial_cmp(&q.point.y).unwrap().reverse());
        let mut active_set = Vec::new();

        for current_sweep_point in 0..points.len() {
            let point = points[current_sweep_point];
            match point.event {
                EventType::Start => {
                    for other in &active_set {
                        num_tests += 1;
                        if let Some(intersect) = line_intersect(point.line, *other) {
                            intersections.push(intersect);
                        }
                    }
                    active_set.push(point.line);
                },
                EventType::End => active_set.retain(|x| *x != point.line),
                _ => {}
            }
        }

        Report { intersections, num_tests }
    }
}

#[derive(Clone, Copy, Debug)]
enum PointKind {
    Start,
    End
}

#[derive(Clone, Copy, Debug)]
struct SweepLineStatus {
    pub line: geometry::Line,
    pub point: geometry::Point,
    pub kind: PointKind
}

impl PartialEq for SweepLineStatus {
    fn eq(&self, other: &Self) -> bool {
        let eps = 10e-3f32;
        f32::abs(self.point.x - other.point.x) < eps && f32::abs(self.point.y - other.point.y) < eps
    }
}

impl Eq for SweepLineStatus {}

impl PartialOrd for SweepLineStatus {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.point.x.partial_cmp(&other.point.x)
    }
}

impl Ord for SweepLineStatus {
    fn cmp(&self, other: &Self) -> Ordering {
        self.point.x.total_cmp(&other.point.x)
    }
}

fn get_neighbors(point: geometry::Point, status: &BTreeSet::<SweepLineStatus>) -> (Option<SweepLineStatus>, Option<SweepLineStatus>) {
    let s_left = SweepLineStatus {
        line: Default::default(),
        point: geometry::Point{ x: point.x - 0.01, y: point.y },
        kind: PointKind::Start
    };
    let s_right = SweepLineStatus {
        line: Default::default(),
        point: geometry::Point{ x: point.x + 0.01, y: point.y },
        kind: PointKind::Start
    };
    (status.range(..s_left).next_back().copied(), status.range(s_right..).next().copied())
}

fn test_neighbour(l1: geometry::Line, l2: geometry::Line, queue: &mut BTreeSet::<SweepLinePoint>) -> Option<Intersection> {
    let i = line_intersect(l1, l2);
    if let Some(intersection) = i {
        queue.insert(SweepLinePoint{
            line: Default::default(),
            point: intersection.point,
            // Todo: add relevant lines to Intersection emote
            event: EventType::Intersection
        });
        println!("Intersection at {:?}", intersection.point);
        return i;
    }

    None
}

impl Intersector for SmartSweepLineIntersector {
    fn report_intersections(lines: &[Line]) -> Report {
        let mut intersections = Vec::new();
        let mut num_tests = 0;

        let mut event_queue = BTreeSet::<SweepLinePoint>::new();
        let mut status = BTreeSet::<SweepLineStatus>::new();

        // Initialize event queue by pushing all points into it.
        for line in lines.into_iter() {
            event_queue.insert(SweepLinePoint {
                line: *line, point: line.a,
                event: match line.a.y > line.b.y {
                    true => EventType::Start,
                    _ => EventType::End
                }
            });
            event_queue.insert(SweepLinePoint {
               line: *line, point: line.b,
                event: match line.b.y > line.a.y {
                    true => EventType::Start,
                    _ => EventType::End
                }
            });
        }

        while let Some(event) = event_queue.pop_first() {
            match event.event {
                EventType::Start => {
                    status.insert( SweepLineStatus {
                        line: event.line,
                        point: event.point,
                        kind: PointKind::Start
                    });
                    let (left, right) = get_neighbors(event.point, &status);
                    if let Some(left) = left {
                        num_tests += 1;
                        if let Some(intersection) = test_neighbour(event.line, left.line, &mut event_queue) {
                            intersections.push(intersection);
                        }
                    }
                    if let Some(right) = right {
                        num_tests += 1;
                        if let Some(intersection) = test_neighbour(event.line, right.line, &mut event_queue) {
                            intersections.push(intersection);
                        }
                    }
                },
                EventType::End => {},
                EventType::Intersection => {},
            };
        }

        Report { intersections, num_tests }
    }
}