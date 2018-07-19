extern crate fuzzy_dbscan;
extern crate rand;

use fuzzy_dbscan::FuzzyDBSCAN;
use rand::{Rng, SeedableRng, StdRng};
use std::f32;

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
    assert_eq!(clusters.len(), 7);
}
