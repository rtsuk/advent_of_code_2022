use euclid::{point2, vec2};
use std::{cmp::Ordering, collections::HashSet};

type Point = euclid::default::Point2D<isize>;
type Vector = euclid::default::Vector2D<isize>;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "L" => Self::Left,
            "R" => Self::Right,
            "U" => Self::Up,
            "D" => Self::Down,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    pub step: Vector,
    pub count: isize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let mut parts = s.split(' ');
        Self {
            step: Vector::from(Direction::from(parts.next().expect("direction"))),
            count: parts
                .next()
                .expect("count")
                .parse::<isize>()
                .expect("usize"),
        }
    }
}

impl From<Direction> for Vector {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => vec2(0, 1),
            Direction::Down => vec2(0, -1),
            Direction::Left => vec2(-1, 0),
            Direction::Right => vec2(1, 0),
        }
    }
}

type MoveList = Vec<Move>;

const DATA: &str = include_str!("../../data/day09.txt");

fn parse(s: &str) -> MoveList {
    s.lines().map(Move::from).collect()
}

fn tail_from_head(head: Point, tail: Point) -> Point {
    let v = head - tail;

    let delta = if v.x == 0 || v.y == 0 {
        let deltax = if v.x > 1 {
            1
        } else if v.x < -1 {
            -1
        } else {
            0
        };

        let deltay = if v.y > 1 {
            1
        } else if v.y < -1 {
            -1
        } else {
            0
        };

        vec2(deltax, deltay)
    } else {
        let deltax = match v.x.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        let deltay = match v.y.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };

        vec2(deltax, deltay)
    };
    let mut new_tail = tail + delta;

    if new_tail == head {
        new_tail = tail;
    }

    new_tail
}

fn execute_moves<const T: usize>(moves: &MoveList) -> usize {
    let mut positions = HashSet::new();

    let mut knots: [Point; T] = [point2(1, 1); T];
    positions.insert(knots[T - 1]);
    for one_move in moves {
        for _ in 0..one_move.count {
            knots[0] += one_move.step;
            for index in 0..T - 1 {
                let trailing = index + 1;
                knots[trailing] = tail_from_head(knots[index], knots[trailing]);
            }
            positions.insert(knots[T - 1]);
        }
    }
    positions.len()
}

fn main() {
    let moves = parse(DATA);
    let positions = execute_moves::<2>(&moves);
    println!("How many positions  = {positions}",);
    let positions = execute_moves::<10>(&moves);
    println!("How many positions(10)  = {positions}",);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    const SAMPLE2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

    #[test]
    fn test_parse() {
        let moves = parse(SAMPLE);
        assert_eq!(moves.len(), 8);
        assert_eq!(
            moves[0],
            Move {
                step: vec2(1, 0),
                count: 4
            }
        );
        assert_eq!(
            moves[3],
            Move {
                step: vec2(0, -1),
                count: 1
            }
        );
    }
    #[test]
    fn test_tail_from_head() {
        let new_tail = tail_from_head(point2(5, 3), point2(4, 1));
        assert_eq!(new_tail, point2(5, 2));
        let new_tail = tail_from_head(point2(4, 5), point2(5, 4));
        assert_eq!(new_tail, point2(5, 4));
    }

    #[test]
    fn test_part_1() {
        let moves = parse(SAMPLE);
        let positions = execute_moves::<2>(&moves);
        assert_eq!(positions, 13);
    }

    #[test]
    fn test_part_2() {
        let moves = parse(SAMPLE);
        let positions = execute_moves::<10>(&moves);
        assert_eq!(positions, 1);

        let moves = parse(SAMPLE2);
        let positions = execute_moves::<10>(&moves);
        assert_eq!(positions, 36);
    }
}
