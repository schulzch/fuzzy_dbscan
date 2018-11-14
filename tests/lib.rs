extern crate fuzzy_dbscan;
extern crate utils;

use fuzzy_dbscan::*;
use utils::*;

const BASE_N: usize = 100;
const BASE_R: f32 = 10.0;

fn density_n(scale: f32) -> usize {
    (BASE_N as f32 * std::f32::consts::PI * scale.powi(2)).round() as usize
}

fn equal_sized_circles() -> Vec<Point> {
    flat_vec![
        uniform_circle(BASE_N, 0.0, 0.0, BASE_R),
        uniform_circle(BASE_N, BASE_R * 5.0, 0.0, BASE_R),
    ]
}

fn barbell_circles() -> Vec<Point> {
    let scale_main = 1.265_f32;
    let scale_link_n = (1.0 / 3.0) as f32; //TODO: magic relation (bug?)
    let scale_link_r = (1.0 / 2.0) as f32; //TODO: magic relation (bug?)
    flat_vec![
        uniform_circle(density_n(scale_main), 0.0, 0.0, BASE_R * scale_main),
        uniform_circle(
            density_n(scale_link_n),
            BASE_R * 2.0,
            0.0,
            BASE_R * scale_link_r
        ),
        uniform_circle(
            density_n(scale_link_n),
            BASE_R * 3.0,
            0.0,
            BASE_R * scale_link_r
        ),
        uniform_circle(
            density_n(scale_main),
            BASE_R * 5.0,
            0.0,
            BASE_R * scale_main
        ),
    ]
}

// FuzzyDBSCAN should reduce to FuzzyCoreDBSCAN (eps_min = eps_max, hard).
#[test]
fn reduce_to_fuzzy_core_dbscan() {
    let points = equal_sized_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R,
        eps_max: BASE_R,
        pts_min: 1.0,
        pts_max: BASE_N as f32,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_core_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should reduce to FuzzyBorderDBSCAN (pts_min = pts_max, hard).
#[test]
fn reduce_to_fuzzy_border_dbscan() {
    let scale = 2.0_f32;
    let points = equal_sized_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R,
        eps_max: BASE_R * scale,
        pts_min: density_n(0.975 * 1.0 / scale) as f32, //TODO: magic number (bug?)
        pts_max: density_n(0.975 * 1.0 / scale) as f32, //TODO: magic number (bug?)
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
        eps_min: BASE_R,
        eps_max: BASE_R,
        pts_min: 1.0,
        pts_max: 1.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy cores.
#[test]
fn vary_cores() {
    let scale = 2.0_f32;
    let points = flat_vec![
        uniform_circle((BASE_N as f32 * scale) as usize, 0.0, 0.0, BASE_R),
        uniform_circle(BASE_N, BASE_R * 5.0, 0.0, BASE_R),
    ];
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R,
        eps_max: BASE_R,
        pts_min: 1.0,
        pts_max: BASE_N as f32 * scale,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_cores", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy borders.
#[test]
fn vary_borders() {
    let scale = 1.33_f32;
    let points = flat_vec![
        uniform_circle(density_n(scale), 0.0, 0.0, BASE_R * scale),
        uniform_circle(
            density_n(scale),
            (BASE_R * scale) * 5.0,
            0.0,
            BASE_R / scale
        ),
    ];
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R / scale,
        eps_max: BASE_R * scale,
        pts_min: density_n(0.94 * 1.0 / scale) as f32, //TODO: magic number (bug?)
        pts_max: density_n(0.94 * 1.0 / scale) as f32, //TODO: magic number (bug?)
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_borders", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying fuzzy cores and borders.
#[test]
fn vary_borders_and_cores() {
    let points = barbell_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R * 0.7,        //TODO: magic number (bug?)
        eps_max: BASE_R * 2.0,        //TODO: magic number (bug?)
        pts_min: BASE_N as f32 * 2.0, //TODO: should depend on scale_xxx
        pts_max: BASE_N as f32 * 5.0, //TODO: should depend on scale_main
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_borders_and_cores", &points, &clusters);
    assert_eq!(clusters.len(), 2);
}

// FuzzyDBSCAN should find varying one DBSCAN-style cluster.
#[test]
fn vary_nothing() {
    let points = barbell_circles();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R * 2.0,
        eps_max: BASE_R * 2.0,
        pts_min: BASE_N as f32 * 2.0,
        pts_max: BASE_N as f32 * 2.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("vary_nothing", &points, &clusters);
    assert_eq!(clusters.len(), 1);
}

// FuzzyDBSCAN should find noise.
#[test]
fn noise() {
    let points = flat_vec![uniform_circle(BASE_N, 0.0, 0.0, BASE_R)];
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R * 2.0,
        eps_max: BASE_R * 4.0,
        pts_min: BASE_N as f32 * 2.0,
        pts_max: BASE_N as f32 * 4.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("noise", &points, &clusters);
    assert_eq!(clusters.len(), 1);
    assert_eq!(clusters[0][0].category, Category::Noise);
}
