use anyhow::Error;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day22.txt");
const SAMPLE: &str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5"#;

#[derive(Debug)]
enum MapCell {
    Void,
    Wall,
    Open,
}

impl From<char> for MapCell {
    fn from(c: char) -> Self {
        match c {
            '.' => MapCell::Open,
            '#' => MapCell::Wall,
            _ => MapCell::Void,
        }
    }
}

type MapRow = Vec<MapCell>;

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>,
}

#[derive(Debug, Clone, Copy)]
enum StepInstruction {
    Go(usize),
    TurnLeft,
    TurnRight,
}

type StepList = Vec<StepInstruction>;

#[derive(Debug)]
struct StepPair(StepInstruction, Option<StepInstruction>);

impl StepPair {
    fn into_vec(self) -> Vec<StepInstruction> {
        if self.1.is_some() {
            vec![self.0, self.1.unwrap()]
        } else {
            vec![self.0]
        }
    }
}

impl From<&str> for StepPair {
    fn from(s: &str) -> Self {
        match s {
            "R" => StepPair(StepInstruction::TurnRight, None),
            "L" => StepPair(StepInstruction::TurnLeft, None),
            _ => {
                let mut second = None;
                let mut num = s;

                if s.ends_with('R') {
                    second = Some(StepInstruction::TurnRight);
                    num = &s[0..s.len() - 1];
                } else if s.ends_with('L') {
                    second = Some(StepInstruction::TurnLeft);
                    num = &s[0..s.len() - 1];
                }
                StepPair(
                    StepInstruction::Go(num.parse::<usize>().expect("go")),
                    second,
                )
            }
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day22", about = "Monkey Map")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn parse(s: &str) -> (Map, StepList) {
    let mut parts = s.split("\n\n");
    let map_text = parts.next().map(str::to_string).expect("map_text");
    let rows: Vec<_> = map_text
        .lines()
        .map(|s| s.chars().map(MapCell::from).collect::<Vec<_>>())
        .collect();
    let path_text = parts.next().map(str::to_string).expect("path_text");
    let path_parts: Vec<_> = path_text
        .split_inclusive(['R', 'L'])
        .map(StepPair::from)
        .flat_map(StepPair::into_vec)
        .collect();

    (Map { rows }, path_parts)
}

fn solve_part_1(_map: &Map, _path: &StepList) -> usize {
    todo!("solve_part_1");
}

fn solve_part_2(_map: &Map, _path: &StepList) -> usize {
    todo!("solve_part_2");
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let (map, path) = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    println!("part 1 password = {}", solve_part_1(&map, &path));

    println!("part 2 password = {}", solve_part_2(&map, &path));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let (_map, _path) = parse(SAMPLE);
        todo!("test_parse");
    }

    #[test]
    #[ignore]
    fn test_part_1() {
        let (_map, _path) = parse(SAMPLE);
        todo!("test_part_1");
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let (_map, _path) = parse(SAMPLE);
        todo!("test_part_2");
    }
}
