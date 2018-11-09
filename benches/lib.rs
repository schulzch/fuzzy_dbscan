#[macro_use]
extern crate criterion;
extern crate fuzzy_dbscan;
extern crate utils;

use fuzzy_dbscan::*;
use utils::*;

use criterion::Criterion;

fn bench_lib(c: &mut Criterion) {
    c.bench_function("100 points", |b| {
        let points = flat_vec![
            uniform_circle(100, 0.0, 0.0, 10.0),
            uniform_circle(100, 50.0, 0.0, 10.0),
        ];
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
