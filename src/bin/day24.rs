#![allow(dead_code)]
use anyhow::Error;
use enum_iterator::{all, Sequence};
use euclid::{point2, size2, vec2};
use pathfinding::prelude::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use structopt::StructOpt;

type Coord = i64;
type Point = euclid::default::Point2D<Coord>;
type UPoint = euclid::default::Point2D<Coord>;
type Box = euclid::default::Box2D<Coord>;
type Vector = euclid::default::Vector2D<Coord>;
type Rect = euclid::default::Rect<Coord>;

const DATA: &str = include_str!("../../data/day24.txt");
const SAMPLE: &str = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"#;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence, Hash)]
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

fn blizzards_from_row((y, cells): (usize, &MapRow)) -> Vec<Blizzard> {
    cells
        .iter()
        .enumerate()
        .map(|(x, cells)| (point2(x, y).to_i64(), cells))
        .filter_map(Blizzard::from_cell)
        .collect()
}

type MapRow = Vec<MapCell>;

#[derive(Debug)]
struct Map {
    bounds: Rect,
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
        let bounds = Rect::new(
            point2(1, 1),
            size2(rows[0].len() - 2, rows.len() - 2).to_i64(),
        );
        Self {
            bounds,
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

    fn blizzard_starts(&self) -> Vec<Blizzard> {
        self.rows
            .iter()
            .enumerate()
            .flat_map(blizzards_from_row)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    position: Point,
    direction: Direction,
}

impl Blizzard {
    fn from_cell((position, cell): (UPoint, &MapCell)) -> Option<Blizzard> {
        match *cell {
            MapCell::Blizzard(direction) => Some(Blizzard {
                position,
                direction,
            }),
            _ => None,
        }
    }

    fn new_pos(&self, map: &Map) -> Self {
        let v: Vector = self.direction.into();
        let position = self.position + v;
        let position = if map.cell_at(&position) == MapCell::Wall {
            match self.direction {
                Direction::North => point2(position.x, map.bounds.max_y() - 1),
                Direction::South => point2(position.x, map.bounds.min_y()),
                Direction::East => point2(map.bounds.min_x(), position.y),
                Direction::West => point2(map.bounds.max_x() - 1, position.y),
            }
        } else {
            position
        };
        Self {
            direction: self.direction,
            position,
        }
    }
}

fn parse(s: &str) -> Map {
    let rows: Vec<_> = s
        .lines()
        .map(|s| s.chars().map(MapCell::from).collect::<Vec<_>>())
        .collect();
    Map::new(rows)
}

#[derive(Debug, Clone)]
struct BlizzardMap {
    blizzards: Vec<Blizzard>,
    blizzard_locations: HashSet<Point>,
}

impl BlizzardMap {
    fn char_for_point(&self, p: &Point) -> Option<char> {
        let blizzards: Vec<char> = self
            .blizzards
            .iter()
            .filter_map(|b| (b.position == *p).then_some(b.direction.into()))
            .collect();

        match blizzards.len() {
            0 => None,
            1 => Some(blizzards[0]),
            _ => Some((b'0' + blizzards.len() as u8) as char),
        }
    }

    fn new(map: &Map) -> Self {
        let blizzards = map.blizzard_starts();
        let blizzard_locations = blizzards.iter().map(|b| b.position).collect();
        Self {
            blizzards,
            blizzard_locations,
        }
    }

    fn new_blizzards(&self, map: &Map) -> Self {
        let blizzards: Vec<Blizzard> = self.blizzards.iter().map(|b| b.new_pos(map)).collect();
        let blizzard_locations = blizzards.iter().map(|b| b.position).collect();
        Self {
            blizzards,
            blizzard_locations,
        }
    }

    fn unique_list(&self, map: &Map) -> Vec<Self> {
        let mut blizzards = self.clone();
        let mut set = HashSet::new();
        let mut list = vec![blizzards.clone()];
        set.insert(blizzards.clone());
        for _ in 0.. {
            let new_blizzards = blizzards.new_blizzards(map);
            if set.contains(&new_blizzards) {
                break;
            }
            set.insert(new_blizzards.clone());
            list.push(new_blizzards.clone());
            blizzards = new_blizzards;
        }
        list
    }
}

impl Hash for BlizzardMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for b in self.blizzards.iter() {
            b.hash(state);
        }
    }
}

impl PartialEq for BlizzardMap {
    fn eq(&self, o: &BlizzardMap) -> bool {
        self.blizzards.eq(&o.blizzards)
    }
}

impl Eq for BlizzardMap {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MapState {
    time: usize,
    blizzards: Rc<Vec<BlizzardMap>>,
    position: Point,
    target: Point,
}

impl MapState {
    fn render(&self, map: &Map) {
        let blizzards = &self.blizzards[self.time % self.blizzards.len()];
        for y in 0..map.rows.len() as Coord {
            let mut s = String::new();
            let row = &map.rows[y as usize];
            for x in 0..row.len() as Coord {
                let pt = point2(x, y);
                let c = if pt == self.position {
                    if blizzards.blizzard_locations.contains(&pt) {
                        '?'
                    } else {
                        'E'
                    }
                } else if let Some(c) = blizzards.char_for_point(&pt) {
                    c
                } else if map.cell_at(&pt) == MapCell::Wall {
                    '#'
                } else {
                    '.'
                };
                s.push(c);
            }
            println!("{s}");
        }
        println!("\n");
    }
}

fn taxicab_distance(p: Point, q: Point) -> Coord {
    let p2 = (p - q).abs();
    p2.x + p2.y
}

fn successors(state: &MapState, map: &Map) -> Vec<(MapState, usize)> {
    let new_time = state.time + 1;
    if new_time % 10 == 0 {
        println!(
            "{new_time} {:?} {}",
            state.position,
            taxicab_distance(state.position, state.target)
        );
    }
    let new_blizzards = &state.blizzards[new_time % state.blizzards.len()];
    all::<Direction>()
        .map(Vector::from)
        .chain(std::iter::once(vec2(0, 0)))
        .filter_map(|v| {
            let new_p = state.position + v;
            let map_cell = map.cell_at(&new_p);
            // println!("new_p = {ne	w_p:?}");
            // println!("map_cell = {map_cell:?}");
            // println!("no_blizzard = {no_blizzard}");
            (map_cell != MapCell::Wall && !new_blizzards.blizzard_locations.contains(&new_p))
                .then_some((
                    MapState {
                        time: new_time,
                        position: new_p,
                        blizzards: state.blizzards.clone(),
                        target: state.target,
                    },
                    1,
                ))
        })
        .collect::<Vec<_>>()
}

fn solve(start: Point, end: Point, map: &Map, start_time: usize) -> usize {
    let blizzards = BlizzardMap::new(map);
    let list = blizzards.unique_list(map);
    let initial_state = MapState {
        blizzards: Rc::new(list),
        time: start_time,
        position: start,
        target: end,
    };
    let path = astar(
        &initial_state,
        |p| successors(p, map),
        |p| taxicab_distance(p.position, end) as usize,
        |state| state.position == state.target,
    )
    .unwrap();

    path.0.len() - 1
}

fn solve_part_1(map: &Map) -> usize {
    solve(map.entrance, map.exit, map, 0)
}

fn solve_part_2(map: &Map, start_time: usize) -> usize {
    let p2_1 = solve(map.exit, map.entrance, map, start_time);
    println!("p2_1 = {p2_1}");
    let p2_2 = solve(map.entrance, map.exit, map, start_time + p2_1);
    println!("p2_2 = {p2_2}");
    p2_1 + p2_2
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day24", about = "Blizzard Basin")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    /// Use presolved part 1
    #[structopt(long)]
    presolved: Option<usize>,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let map = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    let p1 = opt.presolved.unwrap_or_else(|| solve_part_1(&map));
    println!("part 1  = {p1}");

    println!("part 2  = {}", p1 + solve_part_2(&map, p1));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let map = parse(SAMPLE);
        assert_eq!(map.bounds.size, size2(6, 4));
        assert_eq!(map.bounds.origin, point2(1, 1));

        let blizzards = map.blizzard_starts();
        assert_eq!(blizzards.len(), 19);

        assert_eq!(blizzards[0].position, point2(1, 1));
        assert_eq!(blizzards[0].direction, Direction::East);
        assert_eq!(blizzards[1].position, point2(2, 1));
        assert_eq!(blizzards[1].direction, Direction::East);
    }

    #[test]
    fn test_cycle() {
        println!("sample");
        let map = parse(SAMPLE);
        let blizzards = BlizzardMap::new(&map);
        let list = blizzards.unique_list(&map);
        assert_eq!(list.len(), 12);

        println!("data");
        let map = parse(DATA);
        let blizzards = BlizzardMap::new(&map);
        let list = blizzards.unique_list(&map);
        assert_eq!(list.len(), 600);
    }

    #[test]
    fn test_part_1() {
        let map = parse(SAMPLE);
        let p1 = solve_part_1(&map);
        assert_eq!(p1, 18);
    }

    #[test]
    #[ignore]
    fn test_part_2() {}
}
