var FuzzyDBSCAN = require('../pkg/fuzzy_dbscan.js');

var fuzzyDBSCAN = new FuzzyDBSCAN.FuzzyDBSCAN();
fuzzyDBSCAN.eps_min = 10.0;
fuzzyDBSCAN.eps_max = 20.0;
fuzzyDBSCAN.pts_min = 1;
fuzzyDBSCAN.pts_max = 2;

console.log(fuzzyDBSCAN.cluster([{x: 0, y: 0}, {x: 100, y: 100}, {x: 105, y: 105}, {x: 115, y: 115}]));
