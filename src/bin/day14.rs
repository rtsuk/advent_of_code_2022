use anyhow::Error;
use euclid::{point2, vec2};
use std::collections::HashMap;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day14.txt");
const SAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

type Point = euclid::default::Point2D<isize>;
type Vector = euclid::default::Vector2D<isize>;
type Rect = euclid::default::Rect<isize>;
type Box = euclid::default::Box2D<isize>;
type RockList = Vec<Vec<Point>>;

const SAND_ORIGIN: Point = point2(500, 0);

struct LineIter {
    current: Point,
    end: Point,
    delta: Vector,
}

impl LineIter {
    fn new(start: Point, end: Point) -> Self {
        let b = Box::from_points(&[start, end]);
        let start = b.min;
        let end = b.max;
        let mut delta = end - start;
        if delta.x > 0 {
            delta.x /= delta.x;
        }
        if delta.y > 0 {
            delta.y /= delta.y;
        }
        Self {
            current: start,
            delta,
            end,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.x > self.end.x || self.current.y > self.end.y {
            return None;
        }
        let next = self.current;
        self.current += self.delta;
        Some(next)
    }
}

#[derive(Debug)]
enum Block {
    Rock,
    Sand,
}

#[derive(Debug)]
struct RockFall {
    bounds: Rect,
    blocks: HashMap<Point, Block>,
    falling_sand: Option<Point>,
    floor: isize,
    units: usize,
}

impl RockFall {
    fn new(list: RockList, floor: isize) -> Self {
        let bounds = Rect::from_points(list.iter().flatten());
        let mut blocks = HashMap::new();
        for rock in list {
            for i in 0..rock.len() - 1 {
                let iter = LineIter::new(rock[i], rock[i + 1]).map(|p| (p, Block::Rock));
                blocks.extend(iter);
            }
        }
        Self {
            bounds,
            blocks,
            falling_sand: Some(SAND_ORIGIN),
            floor: floor.max(bounds.max_y() + 2),
            units: 1,
        }
    }

    fn step(&mut self) -> Option<usize> {
        const DELTAS: &[Vector] = &[vec2(0, 1), vec2(-1, 1), vec2(1, 1)];
        if let Some(falling_sand) = self.falling_sand.as_mut() {
            for delta in DELTAS {
                let new_pos = *falling_sand + *delta;
                if new_pos.y != self.floor && !self.blocks.contains_key(&new_pos) {
                    *falling_sand = new_pos;
                    if new_pos.y < self.bounds.max_y() + 10 {
                        return None;
                    } else {
                        return Some(self.units - 1);
                    }
                }
            }
            self.blocks.insert(*falling_sand, Block::Sand);
            if *falling_sand == SAND_ORIGIN {
                return Some(self.units);
            }
            *falling_sand = SAND_ORIGIN;
            self.units += 1;
            return None;
        }
        None
    }
}

fn parse_point(s: &str) -> Point {
    let mut parts = s
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::ok)
        .map(Option::unwrap_or_default);

    point2(
        parts.next().unwrap_or_default(),
        parts.next().unwrap_or_default(),
    )
}

fn parse(s: &str) -> RockList {
    s.lines()
        .map(|s| s.split(" -> ").map(parse_point).collect::<Vec<_>>())
        .collect()
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day14", about = "Falling sand.")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    /// No graphics
    #[structopt(long)]
    headless: bool,

    /// Floor level
    #[structopt(long, default_value = "11")]
    floor: isize,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let rocklist = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    let mut rockfall = RockFall::new(rocklist, opt.floor);

    if opt.headless {
        loop {
            if let Some(units) = rockfall.step() {
                println!("units = {units}");
                break;
            }
        }
    } else {
        todo!();
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use euclid::rect;

    #[test]
    fn test_parse() {
        let l = parse(SAMPLE);
        assert_eq!(
            l,
            vec![
                vec![point2(498, 4), point2(498, 6), point2(496, 6)],
                vec![
                    point2(503, 4),
                    point2(502, 4),
                    point2(502, 9),
                    point2(494, 9)
                ]
            ]
        );

        let rockfall = RockFall::new(l, isize::MAX);
        assert_eq!(rockfall.bounds, rect(494, 4, 9, 5));
    }

    #[test]
    fn test_line_iter() {
        let points: Vec<_> = LineIter::new(point2(498, 4), point2(498, 6)).collect();
        dbg!(&points);
        assert_eq!(points, [point2(498, 4,), point2(498, 5,), point2(498, 6,)]);
        let points: Vec<_> = LineIter::new(point2(498, 6), point2(496, 6)).collect();
        assert_eq!(points, [point2(496, 6,), point2(497, 6,), point2(498, 6,)]);
    }

    #[test]
    fn test_part_1() {
        let l = parse(SAMPLE);
        let mut rockfall = RockFall::new(l, isize::MAX);
        loop {
            if let Some(amount) = rockfall.step() {
                assert_eq!(amount, 24);
                break;
            }
        }
    }

    #[test]
    fn test_part_2() {
        let l = parse(SAMPLE);
        let mut rockfall = RockFall::new(l, 0);
        loop {
            if let Some(amount) = rockfall.step() {
                assert_eq!(amount, 93);
                break;
            }
        }
    }
}
