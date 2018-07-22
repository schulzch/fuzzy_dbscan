extern crate fuzzy_dbscan;
extern crate rand;
extern crate svg;

use fuzzy_dbscan::{Category, Cluster, FuzzyDBSCAN};
use rand::{Rng, SeedableRng, StdRng};
use std::f32;
use svg::node::element::Circle;
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

fn default_points() -> Vec<Point> {
    [
        &uniform_circle(100, 1.0, 1.0, 10.0)[..],
        &uniform_circle(100, 100.0, 100.0, 10.0)[..],
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
        // ColorBrewer 12-class paired.
        "#a6cee3",
        "#1f78b4",
        "#b2df8a",
        "#33a02c",
        "#fb9a99",
        "#e31a1c",
        "#fdbf6f",
        "#ff7f00",
        "#cab2d6",
        "#6a3d9a",
        "#ffff99",
        "#b15928",
    ];
    for (cluster_index, cluster) in clusters.iter().enumerate() {
        for assignment in cluster {
            let point = &points[assignment.index];
            let color = if let Category::Noise = assignment.category {
                "#000000"
            } else {
                colors[cluster_index % 12]
            };
            let circle = Circle::new()
                .set("fill", color)
                .set("fill-opacity", assignment.label / 5.0 * 4.0 + 0.2)
                .set("r", 1)
                .set("cx", point.x)
                .set("cy", point.y);

            doc = doc.add(circle);
        }
    }
    println!("{:?}", clusters);

    svg::save(format!("target/_{}.svg", name), &doc).expect("Writing SVG failed");
}

// FuzzyDBSCAN should reduce to classic DBSCAN (eps_min = eps_max, pts_min = pts_max, hard).
#[test]
fn reduce_to_classic() {
    let points = default_points();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 10.0,
        pts_min: 50.0,
        pts_max: 50.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_classic", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should reduce to FuzzyBorderDBSCAN (pts_min = pts_max, hard).
#[test]
fn reduce_to_fuzzy_border() {
    let points = default_points();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 20.0,
        pts_min: 50.0,
        pts_max: 50.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_border", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should reduce to FuzzyCoreDBSCAN (eps_min = eps_max, hard).
#[test]
fn reduce_to_fuzzy_core() {
    let points = default_points();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 10.0,
        pts_min: 1.0,
        pts_max: 100.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_core", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find fuzzy cores.
#[test]
fn find_fuzzy_cores() {
    let points = [
        &uniform_circle(40, 0.0, 0.0, 10.0)[..],
        &uniform_circle(80, 100.0, 0.0, 10.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 10.0,
        eps_max: 10.0,
        pts_min: 50.0,
        pts_max: 70.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("find_fuzzy_cores", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find fuzzy borders.
#[test]
fn find_fuzzy_borders() {
    let points = [
        &uniform_circle(40, 0.0, 0.0, 10.0)[..],
        &uniform_circle(80, 100.0, 0.0, 10.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 50.0,
        eps_max: 90.0,
        pts_min: 50.0,
        pts_max: 50.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("find_fuzzy_borders", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find fuzzy cores and borders.
#[test]
fn find_fuzzy_cores_and_borders() {
    let points = [
        &uniform_circle(40, 30.0, 0.0, 15.0)[..],
        &uniform_circle(10, 50.0, 0.0, 10.0)[..],
        &uniform_circle(40, 70.0, 0.0, 15.0)[..],
    ].concat();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 4.0,
        eps_max: 6.0,
        pts_min: 3.0,
        pts_max: 10.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("find_fuzzy_cores_and_borders", &points, &clusters);
    assert_eq!(clusters.len(), 7);
}
