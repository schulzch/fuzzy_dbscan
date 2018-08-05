# fuzzy_dbscan [![Crates.io](https://img.shields.io/crates/v/fuzzy_dbscan.svg)](https://crates.io/crates/fuzzy_dbscan) [![Docs.rs](https://docs.rs/fuzzy_dbscan/badge.svg)](https://docs.rs/fuzzy_dbscan)

An implementation of the FuzzyDBSCAN algorithm [1].

FuzzyDBSCAN is an agglomerative [fuzzy clustering](https://en.wikipedia.org/wiki/Fuzzy_clustering) algorithm that groups set of points in such a way that one point can belong to more than one group. A points degree of membership is expressed as a category (core, border, noise) and a soft label (between 0.0 and 1.0).

![Example](https://github.com/schulzch/fuzzy_dbscan/raw/master/doc/example.svg?sanitize=true)

## Usage

See [documentation](https://docs.rs/fuzzy_dbscan) for an example.

## References

[1] Dino Ienco, and Gloria Bordogna. "Fuzzy extensions of the DBScan clustering algorithm." Soft Computing (2016).

## Versioning

This project is maintained under the [Semantic Versioning](http://semver.org/) guidelines.

## License

Licensed under the [Apache 2.0 License](https://www.apache.org/licenses/LICENSE-2.0). Copyright &copy; 2018 Christoph Schulz.
