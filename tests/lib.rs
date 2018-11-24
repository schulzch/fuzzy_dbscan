//! Note that the paper contains quite a few errors:
//! - off-by-one inequalities
//! - contradicting formulas
//! - contradicting statements about reductions
//! - ...quality science paper, wtf!
//!
//! Thus, the following tests are based on simplified definitions:
//! - a point is a core point if n other points exist in the local neighbourhood of that point
//! - fuzzy core points belong to the same cluster if they share local neighbourhood
//! - a point is a border point if has less than n other points and at least one fuzzy core point in local neighbourhood
//! - a point is considered noise if it is neither core nor border.
//! - core fuzziness is controlled by pts_min and pts_max
//! - border-fuzziness is controlled by eps_min and eps_max
extern crate fuzzy_dbscan;
extern crate utils;

use fuzzy_dbscan::*;
use utils::*;

#[macro_export]
macro_rules! assert_any {
    ( $clusters:ident, $pred:expr, $res:expr ) => {
        assert!(
            $clusters.iter().all(|ref c| c.iter().any($pred)) == $res,
            concat!("(", stringify!($pred), ") != ", stringify!($res))
        );
    };
}

const BASE_N: usize = 100;
const BASE_R: f32 = 10.0;

fn unimodal_gaussian() -> Vec<Point> {
    flat_vec![gaussian_circle(BASE_N, 0.0, 0.0, BASE_R)]
}

fn bimodal_gaussian() -> Vec<Point> {
    flat_vec![
        gaussian_circle(BASE_N, 0.0, 0.0, BASE_R),
        gaussian_circle(BASE_N, BASE_R * 2.0, 0.0, BASE_R),
        //TODO: more intersection! (bug?)
    ]
}

// FuzzyDBSCAN should reduce to DBSCAN (eps_min = eps_max, pts_min = pts_max), i.e.,
// clusters should have crisp cores only.
#[test]
fn reduce_to_dbscan() {
    let points = unimodal_gaussian();
    // Expect that within the radius should be at least one point.
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R,
        eps_max: BASE_R,
        pts_min: 1.0,
        pts_max: 1.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 1);
    assert_any!(clusters, |ref a| a.label != 1.0, false);
    assert_any!(clusters, |ref a| a.category != Category::Core, false);
}

// FuzzyDBSCAN should reduce to FuzzyCoreDBSCAN (eps_min = eps_max), i.e.,
// clusters should have fuzzy cores and no borders.
#[test]
fn reduce_to_fuzzy_core_dbscan() {
    let points = unimodal_gaussian();
    // Expect that within the radius there can be between one and 100% of the points,
    // thus fuzzy cores will be between 0.5 and 1.0.
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R,
        eps_max: BASE_R,
        pts_min: 1.0,
        pts_max: BASE_N as f32,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_core_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 1);
    assert_any!(
        clusters,
        |ref a| a.category == Category::Core && a.label != 1.0,
        true
    );
    assert_any!(clusters, |ref a| a.category == Category::Border, false);
    assert_any!(clusters, |ref a| a.category == Category::Noise, false);
}

// FuzzyDBSCAN should reduce to FuzzyBorderDBSCAN (pts_min = pts_max), i.e.,
// clusters should have crisp cores and fuzzy borders.
#[test]
fn reduce_to_fuzzy_border_dbscan() {
    let points = unimodal_gaussian();
    // Expect that crisp core points are close to 50% of the points within the
    // maximum radius, thus less than 50% neighbourhood means border.
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 1.0,
        eps_max: BASE_R,
        pts_min: (BASE_N / 2) as f32,
        pts_max: (BASE_N / 2) as f32,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("reduce_to_fuzzy_border_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 1);
    assert_any!(
        clusters,
        |ref a| a.category == Category::Core && a.label != 1.0,
        false
    );
    assert_any!(clusters, |ref a| a.category == Category::Border, true);
}

// FuzzyDBSCAN should find varying fuzzy cores and borders.
#[test]
fn full_fuzzy_dbscan() {
    let points = bimodal_gaussian();
    // Expect that within the radius there can be between 50% and 100% of the
    // points, thus there will be fuzzy cores. Moreover, expect that border
    // points (<50% points) in the "bimodal valley" between the gaussians will be
    // assigned to both clusters.
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: 1.0,
        eps_max: BASE_R,
        pts_min: (BASE_N / 2) as f32,
        pts_max: BASE_N as f32,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("full_fuzzy_dbscan", &points, &clusters);
    assert_eq!(clusters.len(), 2);
    assert_any!(
        clusters,
        |ref a| a.category == Category::Core && a.label != 1.0,
        true
    );
    assert_any!(
        clusters,
        |ref a| a.category == Category::Border && a.label != 1.0,
        true
    );
    assert_any!(clusters, |ref a| a.category == Category::Noise, false);
}

// FuzzyDBSCAN should find noise.
#[test]
fn noise() {
    let points = unimodal_gaussian();
    let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
        distance_fn: &euclidean_distance,
        eps_min: BASE_R * 2.0,
        eps_max: BASE_R * 4.0,
        pts_min: BASE_N as f32 * 2.0,
        pts_max: BASE_N as f32 * 4.0,
    };
    let clusters = fuzzy_dbscan.cluster(&points);
    dump_svg("noise", &points, &clusters);
    assert_eq!(clusters.len(), 1);;
    assert_any!(clusters, |ref a| a.category != Category::Noise, false);
}
