use anyhow::Error;
use enum_iterator::{cardinality, Sequence};
use euclid::{point2, size2, vec2};
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};
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

const NORTH_ADJ_V: [Vector; 3] = [vec2(-1, -1), vec2(0, -1), vec2(1, -1)];
const SOUTH_ADJ_V: [Vector; 3] = [vec2(-1, 1), vec2(0, 1), vec2(1, 1)];
const WEST_ADJ_V: [Vector; 3] = [vec2(-1, -1), vec2(-1, 0), vec2(-1, 1)];
const EAST_ADJ_V: [Vector; 3] = [vec2(1, -1), vec2(1, 0), vec2(1, 1)];

impl Direction {
    fn adjacents(&self, p: Point) -> [Point; 3] {
        match self {
            Direction::North => [p + NORTH_ADJ_V[0], p + NORTH_ADJ_V[1], p + NORTH_ADJ_V[2]],
            Direction::South => [p + SOUTH_ADJ_V[0], p + SOUTH_ADJ_V[1], p + SOUTH_ADJ_V[2]],
            Direction::West => [p + WEST_ADJ_V[0], p + WEST_ADJ_V[1], p + WEST_ADJ_V[2]],
            Direction::East => [p + EAST_ADJ_V[0], p + EAST_ADJ_V[1], p + EAST_ADJ_V[2]],
        }
    }

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

impl From<usize> for Direction {
    fn from(v: usize) -> Self {
        match v {
            0 => Direction::North,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::East,
            _ => panic!("illegal direction"),
        }
    }
}

const DIRECTION_COUNT: usize = cardinality::<Direction>();

type Proposal = Option<Direction>;
type ProposalList = Vec<Proposal>;
type LocationMap = HashMap<Point, usize>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Elf {
    position: Point,
}

impl Elf {
    fn propose(&self, world: &World) -> Proposal {
        let surrounds = Rect::new(self.position - vec2(1, 1), size2(3, 3));
        if world.elf_in_rect(&self.position, &surrounds) {
            'direction: for direction_index in world.time..world.time + DIRECTION_COUNT {
                let direction: Direction = (direction_index % DIRECTION_COUNT).into();
                for p in direction.adjacents(self.position) {
                    if world.elf_at(p) {
                        continue 'direction;
                    }
                }
                return Some(direction);
            }
        }
        None
    }

    fn apply_proposal(&mut self, proposal: Proposal, locations_map: &LocationMap) {
        if let Some(direction) = proposal {
            let delta: Vector = direction.into();
            let new_position = self.position + delta;
            if locations_map
                .get(&new_position)
                .copied()
                .unwrap_or_default()
                <= 1
            {
                self.position = new_position;
            } else {
                // println!("collision at {new_position:?}");
            }
        }
    }

    fn calculate_proposal(&self, proposal: Proposal) -> Point {
        proposal
            .map(|direction| {
                let delta: Vector = direction.into();
                self.position + delta
            })
            .unwrap_or(self.position)
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, o: &Elf) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl Ord for Elf {
    fn cmp(&self, o: &Elf) -> Ordering {
        let x_ord = self.position.x.cmp(&o.position.x);
        match x_ord {
            Ordering::Equal => self.position.y.cmp(&o.position.y),
            _ => x_ord,
        }
    }
}

fn direction_list(time: usize) -> String {
    (time..time + DIRECTION_COUNT)
        .map(|direction_index| {
            let direction: Direction = (direction_index % DIRECTION_COUNT).into();
            let c: char = direction.into();
            c
        })
        .collect::<String>()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct World {
    elves: Vec<Elf>,
    time: usize,
}

impl World {
    fn elf_at(&self, p: Point) -> bool {
        let is_elf = self.elves.iter().any(|elf| elf.position == p);
        // println!("elf_at {p:?} {is_elf}");
        is_elf
    }

    fn elf_in_rect(&self, ignore: &Point, r: &Rect) -> bool {
        self.elves
            .iter()
            .any(|elf| elf.position != *ignore && r.contains(elf.position))
    }

    fn proposals(&self) -> ProposalList {
        self.elves.iter().map(|e| e.propose(self)).collect()
    }

    fn apply_proposals(&mut self, proposals: ProposalList) {
        let new_locations: Vec<Point> = self
            .elves
            .iter()
            .zip(proposals.iter().copied())
            .map(|(e, p)| e.calculate_proposal(p))
            .collect();
        let mut locations_map: LocationMap = HashMap::new();
        for p in new_locations {
            let entry = locations_map.entry(p).or_default();
            *entry += 1;
        }
        self.elves
            .iter_mut()
            .zip(proposals.iter().copied())
            .for_each(|(e, p)| e.apply_proposal(p, &locations_map));
    }

    fn step(&mut self) {
        self.time += 1;
    }

    fn empty_spaces(&self) -> usize {
        let bbox_size = self.bounding_box().size().to_usize();
        (bbox_size.width + 1) * (bbox_size.height + 1) - self.elves.len()
    }

    fn render(&self) {
        let empty_proposals = vec![None; self.elves.len()];
        self.render_with_proposals(&empty_proposals);
    }

    fn render_with_proposals(&self, proposals: &ProposalList) {
        println!(
            "~~~ time = {:2} ~~~ {}",
            self.time,
            direction_list(self.time)
        );
        render_elves(&self.elves, proposals);
    }

    fn bounding_box(&self) -> Box {
        Box::from_points(self.elves.iter().map(|e| e.position))
    }
}

fn render_elves(elves: &Vec<Elf>, proposals: &ProposalList) {
    let bbox = Box::from_points(elves.iter().map(|e| e.position));
    let elf_map: HashMap<_, _> = elves
        .iter()
        .zip(proposals.iter())
        .map(|(e, p)| (e.position, (e, p)))
        .collect();
    for y in bbox.min.y - 2..bbox.max.y + 2 {
        let mut s = format!("{y:04}");
        for x in bbox.min.x - 2..bbox.max.x + 2 {
            let elf_at = elf_map.get(&point2(x, y));
            let c = if let Some((_e, p)) = elf_at {
                if let Some(d) = p {
                    d.as_char()
                } else {
                    '#'
                }
            } else {
                '.'
            };
            s.push(c);
        }
        println!("{}", s);
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day23", about = "Unstable Diffusion")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn maybe_elf(x: isize, y: isize, c: char) -> Option<Elf> {
    (c == '#').then_some(Elf {
        position: point2(x, y).to_i64(),
    })
}

fn handle_line((y, line): (isize, &str), delta_x: isize) -> Vec<Elf> {
    line.chars()
        .enumerate()
        .filter_map(|(x, c)| maybe_elf(x as isize - delta_x, y, c))
        .collect()
}

fn parse(s: &str) -> World {
    let elves: Vec<Elf> = s
        .lines()
        .enumerate()
        .flat_map(|(y, s)| handle_line((y as isize, s), 0))
        .collect();
    World { elves, time: 0 }
}

fn solve_part_1(world: &mut World, expected: Option<&Vec<Vec<Elf>>>, print: bool) -> usize {
    let empty_proposals = vec![None; world.elves.len()];
    for i in 0..10 {
        let time = i + 1;
        if print {
            println!("~~~ Before Round {time}");
            world.render();
        }
        let proposals = world.proposals();
        if print {
            world.render_with_proposals(&proposals);
        }
        if proposals.iter().any(Option::is_some) {
            world.apply_proposals(proposals);
        } else {
            break;
        }
        world.step();
        if print {
            println!("~~~ After Round {time}");
            world.render();
        }
        if let Some(expected) = expected.as_ref() {
            if expected.len() > i {
                let e_set: BTreeSet<_> = expected[i].iter().collect();
                let w_set: BTreeSet<_> = world.elves.iter().collect();
                println!("~~~ expected");
                render_elves(&expected[i], &empty_proposals);
                itertools::assert_equal(e_set.iter(), w_set.iter());
            }
        }
    }
    world.empty_spaces()
}

fn solve_part_2(world: &mut World) -> usize {
    loop {
        let proposals = world.proposals();
        if proposals.iter().any(Option::is_some) {
            world.apply_proposals(proposals);
        } else {
            return world.time + 1;
        }
        world.step();
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut world = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    let mut world2 = world.clone();

    let p1 = solve_part_1(&mut world, None, false);
    println!("part 1 password = {}", p1);

    println!("part 2 password = {}", solve_part_2(&mut world2));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

	const EXPECTED_5: &str = include_str!("../../data/day23_ex.txt");
	const EXPECTED_10: &str = r#"xxx
	.......#......
	...........#..
	..#.#..#......
	......#.......
	...#.....#..#.
	.#......##....
	.....##.......
	..#........#..
	....#.#..#....
	..............
	....#..#..#...
	..............

	"#;

	fn parse_expected(s: &str) -> Vec<Vec<Elf>> {
	    let mut exp = vec![];
	    for chunk in s.split("\n\n") {
	        let mut elves = vec![];
	        for (y, line) in chunk.lines().enumerate().skip(1) {
	            elves.extend(handle_line((y as isize - 3, line), 3));
	        }
	        exp.push(elves);
	    }
	    exp
	}

    #[test]
    fn test_parse() {
        let world = parse(SAMPLE);
        assert_eq!(world.time, 0);
        assert_eq!(world.elves.len(), 22);
        assert_eq!(world.elves[0].position, point2(4, 0));

        let expected = parse_expected(EXPECTED_5);
        dbg!(&expected);
        assert_eq!(expected.len(), 5);
    }

    #[test]
    fn test_part_1() {
        let mut world = parse(SAMPLE);
        let expected = parse_expected(EXPECTED_5);
        let empty_spaces = solve_part_1(&mut world, Some(&expected), true);
        println!("### final expected\n{EXPECTED_10}");
        assert_eq!(empty_spaces, 110);
    }

    #[test]
    fn test_part_2() {
        let mut world = parse(SAMPLE);
        let rounds = solve_part_2(&mut world);
        assert_eq!(rounds, 20);
    }
}

