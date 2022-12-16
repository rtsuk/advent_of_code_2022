use anyhow::Error;
use euclid::point2;
use ranges::{GenericRange, Ranges};
use regex::Regex;
use std::ops::{Bound, RangeBounds, RangeInclusive};
use structopt::StructOpt;

type Coord = i128;
type Point = euclid::default::Point2D<Coord>;

type ImpossibleRange = RangeInclusive<Coord>;

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
    distance: Coord,
}

impl Sensor {
    fn new(location: Point, closest: Point) -> Self {
        Self {
            location,
            distance: taxicab_distance(location, closest),
        }
    }

    fn impossible_range(&self, y: Coord) -> Option<ImpossibleRange> {
        let distance_to_row = (self.location.y - y).abs();
        (distance_to_row < self.distance).then(|| {
            let remaining = self.distance - distance_to_row;
            let x = self.location.x;
            let l_x = x - remaining;
            let h_x = x + remaining;
            l_x..=h_x
        })
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

fn convert_to_inclusive_range(gr: &GenericRange<Coord>) -> ImpossibleRange {
    let start = match gr.start_bound() {
        Bound::Included(t) => *t,
        _ => panic!("unhandled start bound"),
    };
    let end = match gr.end_bound() {
        Bound::Excluded(t) => *t - 1,
        Bound::Included(t) => *t,
        _ => panic!("unhandled end bound"),
    };
    start..=end - 1
}

fn impossible_ranges_with_limit(
    row: Coord,
    limit: Option<Coord>,
    sensors: &[Sensor],
) -> Vec<ImpossibleRange> {
    let impossible_ranges: Vec<_> = sensors
        .iter()
        .filter_map(|sensor| sensor.impossible_range(row))
        .collect();

    let mut ranges = Ranges::new();
    for range in impossible_ranges {
        ranges.insert(range);
    }

    if let Some(limit) = limit {
        ranges = ranges.intersect(0..limit);
    }

    ranges
        .as_slice()
        .iter()
        .map(convert_to_inclusive_range)
        .collect()
}

fn impossible_ranges(row: Coord, sensors: &[Sensor]) -> Vec<ImpossibleRange> {
    impossible_ranges_with_limit(row, None, sensors)
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

const FM: Coord = 4_000_000;

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let sensors = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    let ranges = impossible_ranges(opt.row, &sensors);
    assert_eq!(ranges.len(), 1);
    let r1 = &ranges[0];
    let len = r1.end() - r1.start() + 1;
    println!("impossible_locations len = {len}");

    let limit = opt.max_x + 1;
    for y in 0..limit {
        let ranges = impossible_ranges_with_limit(y, Some(limit), &sensors);
        if ranges.len() > 1 {
            let x = ranges[1].start() - 1;
            println!("found one in row {y}, col {x}, f = {}", x * FM + y);
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let sensors = parse(SAMPLE);
        assert_eq!(sensors.len(), 14);
    }

    #[test]
    fn test_taxicab_distance() {
        let sensors = parse(SAMPLE);

        let sensor = &sensors[0];
        assert_eq!(sensor.distance, 7);

        let sensor = &sensors[4];
        assert_eq!(sensor.distance, 4);
    }

    #[test]
    fn test_impossible_range() {
        let sensors = parse(SAMPLE);
        let sensor = &sensors[6];
        assert_eq!(sensor.location, point2(8, 7));
        let r = sensor.impossible_range(10);
        assert_eq!(r, Some(2..=14));
        let r = sensor.impossible_range(4);
        assert_eq!(r, Some(2..=14));
        let r = sensor.impossible_range(5);
        assert_eq!(r, Some(1..=15));

        let ranges = impossible_ranges(11, &sensors);
        assert_eq!(ranges.len(), 2);

        let ranges = impossible_ranges(10, &sensors);
        assert_eq!(ranges.len(), 1);
    }

    #[test]
    fn test_part_1() {
        let sensors = parse(SAMPLE);
        let ranges = impossible_ranges(10, &sensors);
        assert_eq!(ranges.len(), 1);
        let r1 = &ranges[0];
        let len = r1.end() - r1.start() + 1;
        assert_eq!(len, 26);
    }

    #[test]
    fn test_part_2() {
        let sensors = parse(SAMPLE);
        let ranges = impossible_ranges_with_limit(11, Some(21), &sensors);
        assert_eq!(ranges.len(), 2);
    }
}
