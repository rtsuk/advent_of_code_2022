#![allow(unused)]

use anyhow::Error;
use euclid::{point2, vec2};
use regex::Regex;
use std::collections::{BTreeSet, HashSet};
use structopt::StructOpt;

type Coord = i128;
type Point = euclid::default::Point2D<Coord>;
type Vector = euclid::default::Vector2D<Coord>;
type Box = euclid::default::Box2D<Coord>;

const DATA: &str = include_str!("../../data/day15.txt");
const SAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

fn taxicab_distance(p: Point, q: Point) -> Coord {
    (p - q).abs().to_array().into_iter().sum()
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    closest: Point,
    distance: Coord,
}

impl Sensor {
    fn new(location: Point, closest: Point) -> Self {
        Self {
            location,
            closest,
            distance: taxicab_distance(location, closest),
        }
    }

    fn impossible_location(&self, p: Point) -> bool {
        taxicab_distance(self.location, p) <= self.distance
    }
}

fn point_from_strings(x: &str, y: &str) -> Point {
    point2(
        x.parse::<Coord>().expect("x"),
        y.parse::<Coord>().expect("y"),
    )
}

fn parse(s: &str) -> Vec<Sensor> {
    let re = Regex::new(
        r"Sensor at x=(-*\d+),\s+y=(-*\d+):\s+closest beacon is at x=(-*\d+),\s+y=(-*\d+)",
    )
    .expect("regex");

    re.captures_iter(s)
        .map(|c| {
            Sensor::new(
                point_from_strings(&c[1], &c[2]),
                point_from_strings(&c[3], &c[4]),
            )
        })
        .collect()
}

fn impossible_locations(
    row: Coord,
    max: Option<Coord>,
    include_beacons: bool,
    sensors: &[Sensor],
) -> Vec<Point> {
    let beacons: BTreeSet<_> = sensors
        .iter()
        .filter_map(|s| (s.closest.y == row).then_some(s.closest.x))
        .collect();

    let mut sensor_extent = Box::default();
    for sensor in sensors.iter() {
        let p = sensor.location;
        let d = sensor.distance;
        let vecs: [Vector; 4] = [vec2(-d, 0), vec2(d, 0), vec2(0, d), vec2(0, -d)];
        let points: Vec<_> = vecs.iter().map(|v| p + *v).collect();
        let bounds = Box::from_points(points.iter());
        sensor_extent = sensor_extent.union(&bounds);
    }

    let il: Vec<_> = sensors
        .iter()
        .map(|sensor| {
            let range = if let Some(max) = max {
                0..max
            } else {
                sensor_extent.x_range()
            };
            range
                .filter_map(|x| sensor.impossible_location(point2(x, row)).then_some(x))
                .collect::<HashSet<_>>()
        })
        .collect();

    let mut all_pos = BTreeSet::new();
    for set in il.iter() {
        all_pos.extend(set.iter().copied());
    }

    if !include_beacons {
        for b in beacons.iter() {
            all_pos.remove(b);
        }
    }

    all_pos.iter().map(|x| point2(*x, row)).collect()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day15", about = "Beacon Exclusion Zone")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    #[structopt(short, long, default_value = "10")]
    row: Coord,

    #[structopt(long, default_value = "20")]
    max_x: Coord,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let sensors = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    let impossible_locs = impossible_locations(opt.row, None, false, &sensors);
    println!("impossible_locations len = {}", impossible_locs.len());

    let limit = opt.max_x + 1;
    for y in 0..limit {
        let impossible_locs = impossible_locations(y, Some(limit), true, &sensors);
        println!("y = {y}, impossible_locs.len() = {}", impossible_locs.len());
        if impossible_locs.len() != limit as usize {
            println!("found one in row {y}");
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use euclid::rect;

    #[test]
    fn test_parse() {
        let sensors = parse(SAMPLE);
        assert_eq!(sensors.len(), 14);
    }

    #[test]
    fn test_taxicab_distance() {
        let sensors = parse(SAMPLE);

        let sensor = &sensors[0];
        let distance = taxicab_distance(sensor.location, sensor.closest);
        assert_eq!(distance, 7);

        let sensor = &sensors[4];
        let distance = taxicab_distance(sensor.location, sensor.closest);
        assert_eq!(distance, 4);
    }

    #[test]
    fn test_part_1() {
        let sensors = parse(SAMPLE);

        let impossible_locs = impossible_locations(10, None, false, &sensors);
        assert_eq!(impossible_locs.len(), 26);
    }

    #[test]
    fn test_part_2() {
        let sensors = parse(SAMPLE);
        let impossible_locs = impossible_locations(10, Some(21), true, &sensors);
        println!("row 10, impossible_locs = {:?}", impossible_locs);
        assert_eq!(impossible_locs.len(), 21);
        let impossible_locs = impossible_locations(11, Some(21), true, &sensors);
        println!("row 11, impossible_locs = {:?}", impossible_locs);
        assert_eq!(impossible_locs.len(), 20);
    }
}
