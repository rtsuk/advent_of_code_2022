Here are my Rust solutions to the
[2022 Advent of Code](https://adventofcode.com/2022).

~~~
#![allow(unused)]

use anyhow::Error;
use structopt::StructOpt;

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

fn parse(s: &str) -> () {
    todo!()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day15", about = "Beacon Exclusion Zone")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = Opt::from_args();

    let _ = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use euclid::rect;

    #[test]
    fn test_parse() {
        let l = parse(SAMPLE);
    }


    #[test]
    #[ignore]
    fn test_part_1() {
        let l = parse(SAMPLE);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let l = parse(SAMPLE);
    }
}
~~~