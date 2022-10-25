mod generators;
mod geometry;
mod intersect;

use std::time;
use tiny_skia::*;

fn main() {
    let mut rng = rand::thread_rng();
    let n = 50;
    let lines = generators::generate_lines::<generators::RandomUnitSquare>(n, &mut rng);
    let now = time::Instant::now();
    let report = intersect::report_all_intersections::<intersect::SmartSweepLineIntersector>(&lines);
    let elapsed = now.elapsed();
    println!("#Intersections found: {} (took {:.2?})", report.intersections.len(), elapsed);

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

    for i in report.intersections {
        let path = PathBuilder::from_circle(i.point.x * scale, i.point.y * scale, 5.0).unwrap();
        pixmap.fill_path(&path, &blue, FillRule::Winding, Transform::from_translate(50.0, 50.0), None);
    }

    pixmap.save_png("output.png").unwrap();
}