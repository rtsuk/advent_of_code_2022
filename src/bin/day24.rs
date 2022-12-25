use anyhow::Error;
use enum_iterator::Sequence;
use euclid::vec2;
use structopt::StructOpt;

type Coord = i64;
type Point = euclid::default::Point2D<Coord>;
type Box = euclid::default::Box2D<Coord>;
type Vector = euclid::default::Vector2D<Coord>;
type Rect = euclid::default::Rect<Coord>;

const DATA: &str = include_str!("../../data/day23.txt");
const SAMPLE: &str = r#"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."#;

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

impl Into<Vector> for Direction {
    fn into(self) -> Vector {
        match self {
            Direction::North => vec2(0, -1),
            Direction::East => vec2(1, 0),
            Direction::South => vec2(0, 1),
            Direction::West => vec2(-1, 0),
        }
    }
}

impl Into<char> for Direction {
    fn into(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

fn parse(_s: &str) -> () {}

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
    println!("part 1  = {}", p1);

    println!("part 2  = {}", solve_part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let _ = parse(SAMPLE);
        todo!();
    }

    #[test]
    #[ignore]
    fn test_part_1() {}

    #[test]
    #[ignore]
    fn test_part_2() {}
}
