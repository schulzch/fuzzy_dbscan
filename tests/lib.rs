extern crate fuzzy_dbscan;
extern crate rand;
extern crate svg;

use fuzzy_dbscan::{Category, Cluster, FuzzyDBSCAN};
use rand::{Rng, SeedableRng, StdRng};
use std::f32;
use svg::node::element::{Circle, Definitions, RadialGradient, Stop, Title};
use svg::node::Text;
use svg::Document;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

fn euclidean_distance(a: &Point, b: &Point) -> f32 {
    ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt()
}

fn uniform_circle(n: usize, cx: f32, cy: f32, r: f32) -> Vec<Point> {
    let seed: &[_] = &[1, 2, 3, 4];
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

fn equal_sized_circles() -> Vec<Point> {
    [
        &uniform_circle(100, 0.0, 0.0, 10.0)[..],
        &uniform_circle(100, 50.0, 0.0, 10.0)[..],
    ].concat()
}

fn dump_svg(name: &str, points: &[Point], clusters: &[Cluster]) {
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
                "Cluster {}\n{}\n{:?}\n\n{}",
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

// FuzzyDBSCAN should reduce to FuzzyCoreDBSCAN (eps_min = eps_max, hard).
#[test]
fn reduce_to_fuzzy_core_dbscan() {
    let points = equal_sized_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 10.0,
        pts_min: 1.0,
        pts_max: 100.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_core_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should reduce to FuzzyBorderDBSCAN (pts_min = pts_max, hard).
#[test]
fn reduce_to_fuzzy_border_dbscan() {
    let points = equal_sized_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 15.0,
        pts_min: 75.0,
        pts_max: 75.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_border_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should reduce to DBSCAN (eps_min = eps_max, pts_min = pts_max, hard).
#[test]
fn reduce_to_dbscan() {
    let points = equal_sized_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 10.0,
        pts_min: 50.0,
        pts_max: 50.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy cores.
#[test]
fn vary_cores() {
    let points = [
        &uniform_circle(200, 0.0, 0.0, 10.0)[..],
        &uniform_circle(100, 50.0, 0.0, 10.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 20.0,
        eps_max: 20.0,
        pts_min: 50.0,
        pts_max: 200.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_cores", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy borders.
#[test]
fn vary_borders() {
    let points = [
        &uniform_circle(200, 0.0, 0.0, 15.0)[..],
        &uniform_circle(200, 50.0, 0.0, 5.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 5.0,
        eps_max: 20.0,
        pts_min: 100.0,
        pts_max: 100.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_borders", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy cores and borders.
#[test]
fn vary_borders_and_cores() {
    let points = [
        &uniform_circle(500, 0.0, 0.0, 15.0)[..],
        &uniform_circle(30, 20.0, 0.0, 5.0)[..],
        &uniform_circle(30, 30.0, 0.0, 5.0)[..],
        &uniform_circle(500, 50.0, 0.0, 15.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 7.0,
        eps_max: 20.0,
        pts_min: 200.0,
        pts_max: 500.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_borders_and_cores", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying one DBSCAN-style cluster.
#[test]
fn vary_nothing() {
    let points = [
        &uniform_circle(500, 0.0, 0.0, 15.0)[..],
        &uniform_circle(30, 20.0, 0.0, 5.0)[..],
        &uniform_circle(30, 30.0, 0.0, 5.0)[..],
        &uniform_circle(500, 50.0, 0.0, 15.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 20.0,
        eps_max: 20.0,
        pts_min: 200.0,
        pts_max: 200.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_nothing", &points, &clusters);
    assert_eq!(clusters.len(), 1);
}

// FuzzyDBSCAN should find varying fuzzy cores and borders.
#[test]
fn noise() {
    let points = [&uniform_circle(100, 0.0, 0.0, 20.0)[..]].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 100.0,
        eps_max: 200.0,
        pts_min: 200.0,
        pts_max: 500.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("noise", &points, &clusters);
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0][0].category, Category::Noise);
}
