extern crate fuzzy_dbscan;
extern crate utils;

use fuzzy_dbscan::*;
use utils::*;

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
    let points = flat_vec![
        uniform_circle(200, 0.0, 0.0, 10.0),
        uniform_circle(100, 50.0, 0.0, 10.0),
    ];
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
    let points = flat_vec![
        uniform_circle(200, 0.0, 0.0, 15.0),
        uniform_circle(200, 50.0, 0.0, 5.0),
    ];
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
    let points = flat_vec![
        uniform_circle(500, 0.0, 0.0, 15.0),
        uniform_circle(30, 20.0, 0.0, 5.0),
        uniform_circle(30, 30.0, 0.0, 5.0),
        uniform_circle(500, 50.0, 0.0, 15.0),
    ];
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
    let points = flat_vec![
        uniform_circle(500, 0.0, 0.0, 15.0),
        uniform_circle(30, 20.0, 0.0, 5.0),
        uniform_circle(30, 30.0, 0.0, 5.0),
        uniform_circle(500, 50.0, 0.0, 15.0),
    ];
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
    let points = flat_vec![uniform_circle(100, 0.0, 0.0, 20.0)];
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
