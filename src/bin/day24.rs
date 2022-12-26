#![allow(dead_code)]
use anyhow::Error;
use enum_iterator::Sequence;
use euclid::{point2, vec2};
use structopt::StructOpt;

type Coord = i64;
type Point = euclid::default::Point2D<Coord>;
type Box = euclid::default::Box2D<Coord>;
type Vector = euclid::default::Vector2D<Coord>;
type Rect = euclid::default::Rect<Coord>;

const DATA: &str = include_str!("../../data/day23.txt");
const SAMPLE: &str = r#"#.#####
#.....#
#>....#
#.....#
#...v.#
#.....#
#####.#"#;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence)]
#[repr(usize)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn as_char(&self) -> char {
        (*self).into()
    }
}

impl From<Direction> for Vector {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => vec2(0, -1),
            Direction::East => vec2(1, 0),
            Direction::South => vec2(0, 1),
            Direction::West => vec2(-1, 0),
        }
    }
}

impl From<Direction> for char {
    fn from(val: Direction) -> Self {
        match val {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapCell {
    Blizzard(Direction),
    Wall,
    Open,
}

impl From<char> for MapCell {
    fn from(c: char) -> Self {
        match c {
            '.' => MapCell::Open,
            '#' => MapCell::Wall,
            '^' => MapCell::Blizzard(Direction::North),
            '>' => MapCell::Blizzard(Direction::East),
            'v' => MapCell::Blizzard(Direction::South),
            '<' => MapCell::Blizzard(Direction::West),
            _ => panic!("unknown cell"),
        }
    }
}

type MapRow = Vec<MapCell>;

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>,
    entrance: Point,
    exit: Point,
}

impl Map {
    fn new(rows: Vec<MapRow>) -> Self {
        let entrance = rows[0]
            .iter()
            .enumerate()
            .find(|(_index, cell)| **cell == MapCell::Open)
            .expect("entrance")
            .0;
        let last_row = rows.len() - 1;
        let exit = rows[last_row]
            .iter()
            .enumerate()
            .find(|(_index, cell)| **cell == MapCell::Open)
            .expect("exit")
            .0;
        Self {
            rows,
            entrance: point2(entrance as Coord, 0),
            exit: point2(exit as Coord, last_row as Coord),
        }
    }

    fn cell_at(&self, p: &Point) -> MapCell {
        if p.x < 0 || p.y < 0 {
            return MapCell::Wall;
        }

        let p_u = p.to_usize();

        if p_u.y >= self.rows.len() {
            return MapCell::Wall;
        }

        let row = &self.rows[p_u.y];
        if p_u.x >= row.len() {
            return MapCell::Wall;
        }

        row[p_u.x]
    }
}

fn parse(s: &str) -> Map {
    let rows: Vec<_> = s
        .lines()
        .map(|s| s.chars().map(MapCell::from).collect::<Vec<_>>())
        .collect();
    println!("rows = {rows:?}");
    Map::new(rows)
}

fn solve_part_1() -> usize {
    todo!();
}

fn solve_part_2() -> usize {
    todo!();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day24", about = "Blizzard Basin")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let _ = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    let p1 = solve_part_1();
    println!("part 1  = {p1}");

    println!("part 2  = {}", solve_part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let map = parse(SAMPLE);
        dbg!(&map);
        todo!();
    }

    #[test]
    #[ignore]
    fn test_part_1() {}

    #[test]
    #[ignore]
    fn test_part_2() {}
}
