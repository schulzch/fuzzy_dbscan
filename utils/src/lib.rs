extern crate fuzzy_dbscan;
extern crate rand;
extern crate svg;

use fuzzy_dbscan::{Category, Cluster};
use rand::{Rng, SeedableRng, StdRng};
use std::f32;
use svg::node::element::{Circle, Definitions, RadialGradient, Stop, Title};
use svg::node::Text;
use svg::Document;

#[derive(Clone)]
pub struct Point {
    x: f32,
    y: f32,
}

#[macro_export]
macro_rules! flat_vec {
    [ $x:expr ] => { [ & $x [..] ].concat() };
    [ $( $x:expr , )* ] => { [ $( & $x [..], )* ].concat() };
}

pub fn euclidean_distance(a: &Point, b: &Point) -> f32 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

pub fn uniform_circle(n: usize, cx: f32, cy: f32, r: f32) -> Vec<Point> {
    let mut seed = [0u8; 32];
    seed.copy_from_slice((0..32).map(|i| i + 1).collect::<Vec<u8>>().as_slice());
    let mut random: StdRng = SeedableRng::from_seed(seed);
    let mut points = Vec::new();
    for _ in 0..n {
        let t = 2.0 * f32::consts::PI * random.gen::<f32>();
        let u = random.gen::<f32>() + random.gen::<f32>();
        let uu = if u > 1.0 { 2.0 - u } else { u };
        points.push(Point {
            x: cx + r * uu * t.cos(),
            y: cy + r * uu * t.sin(),
        });
    }
    return points;
}

pub fn dump_svg(name: &str, points: &[Point], clusters: &[Cluster]) {
    let (min_x, min_y, max_x, max_y) = points.iter().cloned().fold(
        (f32::MAX, f32::MAX, f32::MIN, f32::MIN),
        |extrema, point| {
            (
                extrema.0.min(point.x),
                extrema.1.min(point.y),
                extrema.2.max(point.x),
                extrema.3.max(point.y),
            )
        },
    );
    let margin = 5.0;
    let mut doc = Document::new().set(
        "viewBox",
        (
            min_x - margin,
            min_y - margin,
            (max_x - min_x) + 2.0 * margin,
            (max_y - min_y) + 2.0 * margin,
        ),
    );
    #[rustfmt::skip]
    let colors = [
        // Black.
        "#000000",
        // ColorBrewer Set1.
        "#e41a1c",
        "#377eb8",
        "#4daf4a",
        "#984ea3",
        "#ff7f00",
        "#a65628",
        "#f781bf",
    ];
    let mut defs = Definitions::new();
    for (color_index, color) in colors.iter().enumerate() {
        let stop = |x: f32| {
            // Sample a Guassian distribution for more pleasant perception.
            let phi = |x: f32| (1.0 / (2.0 * f32::consts::PI).sqrt() * (-0.5 * (x).powi(2)).exp());
            Stop::new()
                .set("offset", format!("{}%", (x * 100.0).round()))
                .set("stop-opacity", phi(x * 4.0) / 0.4)
                .set("stop-color", color.to_string())
        };
        let gradient = RadialGradient::new()
            .set("id", format!("g{}", color_index))
            .add(stop(0.0))
            .add(stop(0.125))
            .add(stop(0.25))
            .add(stop(0.375))
            .add(stop(0.5))
            .add(stop(0.625))
            .add(stop(0.75))
            .add(stop(0.875))
            .add(stop(1.0));
        defs = defs.add(gradient);
    }
    doc = doc.add(defs);
    for (cluster_index, cluster) in clusters.iter().enumerate() {
        for assignment in cluster {
            let point = &points[assignment.index];
            let color_index = if let Category::Noise = assignment.category {
                0
            } else {
                1 + cluster_index % (colors.len() - 1)
            };
            let text = format!(
                "Cluster: {}\n\nLabel: {:.1}\nCategory: {:?}\nPoint-Index: {}",
                cluster_index, assignment.label, assignment.category, assignment.index
            );
            let circle = Circle::new()
                .set("fill", format!("url(#g{})", color_index))
                .set("fill-opacity", assignment.label / 5.0 * 4.0 + 0.2)
                .set("r", 1)
                .set("cx", point.x)
                .set("cy", point.y)
                .add(Title::new().add(Text::new(text)));
            doc = doc.add(circle);
        }
    }
    //println!("{:?}", clusters);
    svg::save(format!("target/_{}.svg", name), &doc).expect("Writing SVG failed");
}
