#[macro_use]
extern crate criterion;
extern crate fuzzy_dbscan;
extern crate rand;

use criterion::Criterion;
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

fn bench_lib(c: &mut Criterion) {
    c.bench_function("100 points", |b| {
        let points = [
            &uniform_circle(100, 0.0, 0.0, 10.0)[..],
            &uniform_circle(100, 50.0, 0.0, 10.0)[..],
        ].concat();
        let fuzzy_dbscan = FuzzyDBSCAN::<Point> {
            distance_fn: &euclidean_distance,
            eps_min: 20.0,
            eps_max: 20.0,
            pts_min: 50.0,
            pts_max: 50.0,
        };
        b.iter(|| fuzzy_dbscan.cluster(&points))
    });
}

criterion_group!(benches, bench_lib);
criterion_main!(benches);
