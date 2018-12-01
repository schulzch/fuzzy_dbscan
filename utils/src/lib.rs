extern crate fuzzy_dbscan;
extern crate rand;
extern crate svg;

use fuzzy_dbscan::{Category, Cluster, MetricSpace};
use rand::distributions::{Distribution, Normal};
use rand::{SeedableRng, StdRng};
use std::f64;
use svg::node::element::{Circle, Definitions, RadialGradient, Stop, Title};
use svg::node::Text;
use svg::Document;

#[macro_export]
macro_rules! flat_vec {
    [ $x:expr ] => { [ & $x [..] ].concat() };
    [ $( $x:expr , )* ] => { [ $( & $x [..], )* ].concat() };
}

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

impl MetricSpace for Point {
    fn distance(&self, other: &Self) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
    }
}

fn seeded_rng() -> StdRng {
    let mut seed = [0u8; 32];
    seed.copy_from_slice((0..32).map(|i| i + 1).collect::<Vec<u8>>().as_slice());
    SeedableRng::from_seed(seed)
}

pub fn gaussian_circle(n: usize, cx: f64, cy: f64, r: f64) -> Vec<Point> {
    let center = Point { x: cx, y: cy };
    let sigma = r / 3.0;
    let normal_x = Normal::new(cx as f64, sigma as f64);
    let normal_y = Normal::new(cy as f64, sigma as f64);
    let mut random = seeded_rng();
    let mut points = Vec::new();
    let mut c = 0;
    while c < n {
        let sample = Point {
            x: normal_x.sample(&mut random) as f64,
            y: normal_y.sample(&mut random) as f64,
        };
        if center.distance(&sample) <= r {
            points.push(sample);
            c += 1;
        }
    }
    points
}

pub fn dump_svg(name: &str, points: &[Point], clusters: &[Cluster]) {
    let (min_x, min_y, max_x, max_y) = points.iter().cloned().fold(
        (f64::MAX, f64::MAX, f64::MIN, f64::MIN),
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
        let stop = |x: f64| {
            // Guassian apodization function for more pleasant perception.
            let apo = |x: f64| (-x.powi(2) / (2.0 * (1.0 / 3.0_f64).powi(2))).exp();
            Stop::new()
                .set("offset", format!("{}%", (x * 100.0).round()))
                .set("stop-opacity", apo(x))
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
            let opacity = assignment.label * 0.9 + 0.1;
            let color_index = if let Category::Noise = assignment.category {
                0
            } else {
                1 + cluster_index % (colors.len() - 1)
            };
            let stroke_width = if let Category::Core = assignment.category {
                0.01
            } else {
                0.0
            };
            let text = format!(
                "Cluster: {}\n\nLabel: {:.1}\nCategory: {:?}\nPoint-Index: {}\nLocation: {}, {}",
                cluster_index,
                assignment.label,
                assignment.category,
                assignment.index,
                point.x,
                point.y
            );
            let circle = Circle::new()
                .set("fill", format!("url(#g{})", color_index))
                .set("fill-opacity", opacity)
                .set("stroke", "black")
                .set("stroke-width", stroke_width)
                .set("stroke-opacity", opacity)
                .set("r", 0.5)
                .set("cx", point.x)
                .set("cy", point.y)
                .add(Title::new().add(Text::new(text)));
            doc = doc.add(circle);
        }
    }
    //println!("{:?}", clusters);
    svg::save(format!("target/_{}.svg", name), &doc).expect("Writing SVG failed");
}
