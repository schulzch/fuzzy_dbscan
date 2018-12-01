//! An implementation of the FuzzyDBSCAN algorithm.
//!
//! # Example
//!
//! ```rust
//! extern crate fuzzy_dbscan;
//!
//! #[derive(Debug)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! impl fuzzy_dbscan::MetricSpace for Point {
//!     fn distance(&self, other: &Self) -> f32 {
//!         ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt()
//!     }
//! }
//!
//! fn main() {
//!     let points = vec![
//!         Point { x: 0.0, y: 0.0 },
//!         Point { x: 100.0, y: 100.0 },
//!         Point { x: 105.0, y: 105.0 },
//!         Point { x: 115.0, y: 115.0 },
//!     ];
//!
//!     let fuzzy_dbscan = fuzzy_dbscan::FuzzyDBSCAN {
//!         eps_min: 10.0,
//!         eps_max: 20.0,
//!         pts_min: 1.0,
//!         pts_max: 2.0,
//!     };
//!
//!     println!("{:?}", fuzzy_dbscan.cluster(&points));
//! }
//! ```
extern crate wasm_bindgen;
#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;

use std::collections::HashSet;
use std::f32;

fn take_arbitrary(set: &mut HashSet<usize>) -> Option<usize> {
    let value_copy = if let Some(value) = set.iter().next() {
        Some(*value)
    } else {
        None
    };
    if let Some(value) = value_copy {
        set.take(&value)
    } else {
        None
    }
}

/// A trait to compute distances between points.
pub trait MetricSpace: Sized {
    /// Returns the distance between `self` and `other`.
    fn distance(&self, other: &Self) -> f32;
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[derive(Deserialize)]
pub struct JsPoint {
    x: f64,
    y: f64,
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
impl MetricSpace for JsPoint {
    fn distance(&self, other: &Self) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2)).sqrt() as f32
    }
}

/// A high-level classification, as defined by the FuzzyDBSCAN algorithm.
#[derive(PartialEq, Serialize)]
pub enum Category {
    Core,
    Border,
    Noise,
}

/// An element of a [cluster](Cluster).
#[derive(Serialize)]
pub struct Assignment {
    /// The point index.
    pub index: usize,
    /// A (soft) label between `0.0` and `1.0`.
    pub label: f32,
    /// A high-level category.
    pub category: Category,
}

/// A group of [assigned](Assignment) points.
pub type Cluster = Vec<Assignment>;

/// An instance of the FuzzyDBSCAN algorithm.
///
/// Note that when setting `eps_min = eps_max` and `pts_min = pts_max` the algorithm will reduce to classic DBSCAN.
#[wasm_bindgen]
pub struct FuzzyDBSCAN {
    /// The minimum fuzzy local neighborhood radius.
    pub eps_min: f32,
    /// The maximum fuzzy local neighborhood radius.
    pub eps_max: f32,
    /// The minimum fuzzy neighborhood density (number of points).
    pub pts_min: f32,
    /// The maximum fuzzy neighborhood density (number of points).
    pub pts_max: f32,
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
#[wasm_bindgen]
impl FuzzyDBSCAN {
    /// Creates a new instance of the algorithm.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        FuzzyDBSCAN {
            eps_min: f32::NAN,
            eps_max: f32::NAN,
            pts_min: f32::NAN,
            pts_max: f32::NAN,
        }
    }

    /// Clusters a list of `js_points`.
    pub fn cluster(&self, js_points: JsValue) -> JsValue {
        let points: Vec<JsPoint> = js_points.into_serde().unwrap();
        let clusters = self.fuzzy_dbscan(&points);
        JsValue::from_serde(&clusters).unwrap()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl FuzzyDBSCAN {
    /// Clusters a list of `points`.
    pub fn cluster<P: MetricSpace>(&self, points: &[P]) -> Vec<Cluster> {
        self.fuzzy_dbscan(points)
    }
}

impl FuzzyDBSCAN {
    fn fuzzy_dbscan<P: MetricSpace>(&self, points: &[P]) -> Vec<Cluster> {
        let mut clusters = Vec::new();
        let mut noise_cluster = Vec::new();
        let mut visited = vec![false; points.len()];
        for point_index in 0..points.len() {
            if visited[point_index] {
                continue;
            }
            visited[point_index] = true;
            let neighbor_indices = self.region_query(points, point_index);
            let point_label = self.mu_min_p(self.density(point_index, &neighbor_indices, points));
            if point_label == 0.0 {
                noise_cluster.push(Assignment {
                    index: point_index,
                    category: Category::Noise,
                    label: 1.0,
                });
            } else {
                clusters.push(self.expand_cluster_fuzzy(
                    point_label,
                    point_index,
                    neighbor_indices,
                    points,
                    &mut visited,
                ));
            }
        }
        if !noise_cluster.is_empty() {
            clusters.push(noise_cluster);
        }
        clusters
    }

    fn expand_cluster_fuzzy<P: MetricSpace>(
        &self,
        point_label: f32,
        point_index: usize,
        mut neighbor_indices: HashSet<usize>,
        points: &[P],
        visited: &mut [bool],
    ) -> Vec<Assignment> {
        let mut cluster = vec![Assignment {
            index: point_index,
            category: Category::Core,
            label: point_label,
        }];
        let mut border_points = Vec::new();
        let mut neighbor_visited = vec![false; points.len()];
        while let Some(neighbor_index) = take_arbitrary(&mut neighbor_indices) {
            neighbor_visited[neighbor_index] = true;
            visited[neighbor_index] = true;
            let neighbor_neighbor_indices = self.region_query(points, neighbor_index);
            let neighbor_label =
                self.mu_min_p(self.density(neighbor_index, &neighbor_neighbor_indices, points));
            if neighbor_label > 0.0 {
                for neighbor_neighbor_index in neighbor_neighbor_indices {
                    if !neighbor_visited[neighbor_neighbor_index] {
                        neighbor_indices.insert(neighbor_neighbor_index);
                    }
                }
                cluster.push(Assignment {
                    index: neighbor_index,
                    category: Category::Core,
                    label: neighbor_label,
                });
            } else {
                border_points.push(Assignment {
                    index: neighbor_index,
                    category: Category::Border,
                    label: f32::MAX,
                });
            }
        }
        for border_point in &mut border_points {
            for cluster_point in &cluster {
                let mu_distance =
                    self.mu_distance(&points[border_point.index], &points[cluster_point.index]);
                if mu_distance > 0.0 {
                    border_point.label =
                        cluster_point.label.min(mu_distance).min(border_point.label);
                }
            }
        }
        cluster.append(&mut border_points);
        cluster
    }

    fn region_query<P: MetricSpace>(&self, points: &[P], point_index: usize) -> HashSet<usize> {
        points
            .iter()
            .enumerate()
            .filter(|(neighbor_index, neighbor_point)| {
                *neighbor_index != point_index
                    && neighbor_point.distance(&points[point_index]) <= self.eps_max
            }).map(|(neighbor_index, _)| neighbor_index)
            .collect() //TODO: would be neat to prevent this allocation.
    }

    fn density<P: MetricSpace>(
        &self,
        point_index: usize,
        neighbor_indices: &HashSet<usize>,
        points: &[P],
    ) -> f32 {
        1.0 + neighbor_indices.iter().fold(0.0, |sum, &neighbor_index| {
            sum + self.mu_distance(&points[point_index], &points[neighbor_index])
        })
    }

    fn mu_min_p(&self, n: f32) -> f32 {
        if n >= self.pts_max {
            1.0
        } else if n < self.pts_min {
            0.0
        } else {
            (n - self.pts_min) / (self.pts_max - self.pts_min)
        }
    }

    fn mu_distance<P: MetricSpace>(&self, a: &P, b: &P) -> f32 {
        let distance = a.distance(b);
        if distance <= self.eps_min {
            1.0
        } else if distance > self.eps_max {
            0.0
        } else {
            (self.eps_max - distance) / (self.eps_max - self.eps_min)
        }
    }
}
