use anyhow::Error;
use euclid::{point2, vec2};
use structopt::StructOpt;

type Point = euclid::default::Point2D<isize>;
type Vector = euclid::default::Vector2D<isize>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Player {
    position: Point,
    direction: Direction,
}

impl Player {
    fn password(&self) -> isize {
        let one_pos = self.position + vec2(1, 1);
        one_pos.x * 4
            + one_pos.y * 1000
            + match self.direction {
                Direction::North => 3,
                Direction::East => 0,
                Direction::South => 1,
                Direction::West => 2,
            }
    }
}

#[derive(Debug)]
struct Map {
    rows: Vec<MapRow>,
}

impl Map {
    fn new(rows: Vec<MapRow>) -> Self {
        Self { rows }
    }

    fn cell_at(&self, p: &Point) -> MapCell {
        if p.x < 0 || p.y < 0 || p.y >= self.rows.len() as isize {
            return MapCell::Void;
        }

        let p_u = p.to_usize();

        let row = &self.rows[p_u.y];
        if p_u.x >= row.len() {
            return MapCell::Void;
        }

        row[p_u.x]
    }

    fn start_cell(&self) -> Point {
        let row = &self.rows[0];
        (0..row.len())
            .map(|x| point2(x as isize, 0))
            .find(|p| self.cell_at(p) == MapCell::Open)
            .expect("start")
    }

    fn first_non_void_in_row(&self, y: isize) -> (isize, MapCell) {
        self.rows[y as usize]
            .iter()
            .enumerate()
            .find(|(_index, cell)| **cell != MapCell::Void)
            .map(|(index, cell)| (index as isize, *cell))
            .expect("first_non_void_in_row")
    }

    fn last_non_void_in_row(&self, y: isize) -> (isize, MapCell) {
        // println!("last_non_void_in_row {y}");
        let max_x = self.rows[y as usize].len() as isize;
        // println!("max_x {max_x}");
        for x in (0..max_x).rev() {
            let pt = point2(x, y);
            let cell = self.cell_at(&pt);
            if cell != MapCell::Void {
                return (x, cell);
            }
        }
        unreachable!();
    }

    fn first_non_void_in_col(&self, x: isize) -> (isize, MapCell) {
        for y in 0..self.rows.len() as isize {
            let cell = self.cell_at(&point2(x, y));
            if cell != MapCell::Void {
                return (y, cell);
            }
        }
        unreachable!();
    }

    fn last_non_void_in_col(&self, x: isize) -> (isize, MapCell) {
        for y in (0..self.rows.len() as isize).rev() {
            let pt = point2(x, y);
            // println!("pt = {pt:?}");
            let cell = self.cell_at(&pt);
            if cell != MapCell::Void {
                return (y, cell);
            }
        }
        unreachable!();
    }

    fn wrap(&self, pt: &Point, direction: Direction) -> Option<Point> {
        match direction {
            Direction::East => {
                let (x, cell) = self.first_non_void_in_row(pt.y);
                match cell {
                    MapCell::Wall => None,
                    MapCell::Open => Some(point2(x, pt.y)),
                    MapCell::Void => unreachable!(),
                }
            }
            Direction::West => {
                let (x, cell) = self.last_non_void_in_row(pt.y);
                match cell {
                    MapCell::Wall => None,
                    MapCell::Open => Some(point2(x, pt.y)),
                    MapCell::Void => unreachable!(),
                }
            }
            Direction::South => {
                let (y, cell) = self.first_non_void_in_col(pt.x);
                match cell {
                    MapCell::Wall => None,
                    MapCell::Open => Some(point2(pt.x, y)),
                    MapCell::Void => unreachable!(),
                }
            }
            Direction::North => {
                let (y, cell) = self.last_non_void_in_col(pt.x);
                match cell {
                    MapCell::Wall => None,
                    MapCell::Open => Some(point2(pt.x, y)),
                    MapCell::Void => unreachable!(),
                }
            }
        }
    }

    fn execute_step(&self, player: &Player, step: StepInstruction) -> Player {
        match step {
            StepInstruction::TurnLeft => Player {
                direction: player.direction.turn_left(),
                ..*player
            },
            StepInstruction::TurnRight => Player {
                direction: player.direction.turn_right(),
                ..*player
            },
            StepInstruction::Go(distance) => {
                let mut pt = player.position;
                let vec: Vector = player.direction.into();
                for _d in 0..distance {
                    let new_pt = pt + vec;
                    let map_cell = self.cell_at(&new_pt);
                    match map_cell {
                        MapCell::Wall => {
                            break;
                        }
                        MapCell::Open => {
                            pt = new_pt;
                        }
                        MapCell::Void => {
                            if let Some(tele_point) = self.wrap(&pt, player.direction) {
                                pt = tele_point;
                            } else {
                                break;
                            }
                        }
                    }
                }
                Player {
                    position: pt,
                    ..*player
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum StepInstruction {
    Go(usize),
    TurnLeft,
    TurnRight,
}

type StepList = Vec<StepInstruction>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
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

    (Map::new(rows), path_parts)
}

fn solve_part_1(map: &Map, path: &StepList) -> isize {
    let mut player = Player {
        position: map.start_cell(),
        direction: Direction::East,
    };
    for step in path.iter() {
        player = map.execute_step(&player, *step);
        // println!("after execute_step: step = {step:?} player = {player:?}");
    }
    player.password()
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
        let (map, path) = parse(SAMPLE);
        assert_eq!(map.rows.len(), 12);
        assert_eq!(path.len(), 13);

        assert_eq!(map.cell_at(&point2(0, 0)), MapCell::Void);
        assert_eq!(map.cell_at(&point2(9, 0)), MapCell::Open);
        assert_eq!(map.cell_at(&point2(11, 0)), MapCell::Wall);

        assert_eq!(map.start_cell(), point2(8, 0));
    }

    #[test]
    fn test_part_1() {
        let (map, path) = parse(SAMPLE);
        let player = Player {
            position: map.start_cell(),
            direction: Direction::East,
        };
        let new_player = map.execute_step(&player, path[0]);
        assert_eq!(point2(10, 0), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[1]);
        assert_eq!(point2(10, 0), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[2]);
        assert_eq!(point2(10, 5), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[3]);
        assert_eq!(point2(10, 5), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[4]);
        assert_eq!(point2(3, 5), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[5]);
        assert_eq!(point2(3, 5), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[6]);
        assert_eq!(point2(3, 7), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[7]);
        assert_eq!(point2(3, 7), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[8]);
        assert_eq!(point2(7, 7), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[9]);
        assert_eq!(point2(7, 7), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[10]);
        assert_eq!(point2(7, 5), new_player.position);
        assert_eq!(Direction::South, new_player.direction);

        let new_player = map.execute_step(&new_player, path[11]);
        assert_eq!(point2(7, 5), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let new_player = map.execute_step(&new_player, path[12]);
        assert_eq!(point2(7, 5), new_player.position);
        assert_eq!(Direction::East, new_player.direction);

        let password = new_player.password();

        assert_eq!(password, 6032);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let (_map, _path) = parse(SAMPLE);
        todo!("test_part_2");
    }
}
