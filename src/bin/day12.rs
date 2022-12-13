use euclid::{point2, size2, vec2};
use pathfinding::prelude::*;
use std::{
    cell::RefCell,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

const DATA: &str = include_str!("../../data/day12.txt");

type Size = euclid::default::Size2D<isize>;
type Point = euclid::default::Point2D<isize>;
type Rect = euclid::default::Rect<isize>;

fn height_value(c: char) -> usize {
    c as usize - 'a' as usize
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn in_direction(&self, from: Point, bounds: &Rect) -> Option<Point> {
        let p = match self {
            Self::North => from + vec2(0, -1),
            Self::South => from + vec2(0, 1),
            Self::East => from + vec2(1, 0),
            Self::West => from + vec2(-1, 0),
        };
        bounds.contains(p).then_some(p)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Element {
    Start,
    End,
    Height(usize),
}

impl Element {
    fn elevation(&self) -> usize {
        match self {
            Self::Start => height_value('a'),
            Self::End => height_value('z'),
            Self::Height(v) => *v,
        }
    }

    fn is_legal_from(&self, other: &Element) -> bool {
        let my_height = self.elevation();
        let other_height = other.elevation();
        let delta = other_height as isize - my_height as isize;

        delta <= 1
    }
}

impl From<char> for Element {
    fn from(c: char) -> Self {
        match c {
            'S' => Element::Start,
            'E' => Element::End,
            'a'..='z' => Element::Height(height_value(c)),
            _ => panic!("illegal height"),
        }
    }
}

fn direction_char(from: Point, to: Point) -> char {
    let x_cmp = to.x.cmp(&from.x);
    let y_cmp = to.y.cmp(&from.y);

    match x_cmp {
        Ordering::Less => '<',
        Ordering::Greater => '>',
        Ordering::Equal => match y_cmp {
            Ordering::Less => '^',
            Ordering::Greater => 'v',
            Ordering::Equal => panic!("no direction"),
        },
    }
}

#[derive(Debug, Clone)]
struct Map {
    bounds: Rect,
    data: Vec<Vec<Element>>,
    start: Point,
    end: Point,
}

impl Map {
    fn get_element(&self, p: &Point) -> Element {
        self.data[p.y as usize][p.x as usize]
    }

    fn render_result(&self, result: &Vec<Position>, data: &str) -> String {
        let mut lines = vec![];
        for line in data.lines() {
            let mut s = vec![];
            for c in line.chars() {
                s.push(c);
            }
            lines.push(s);
        }

        for i in 0..result.len() - 1 {
            let from = result[i].point;
            let to = result[i + 1].point;
            let c = direction_char(from, to);
            lines[from.y as usize][from.x as usize] = c;
        }
        lines
            .iter()
            .map(|s| s.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

type MapPtr = Rc<RefCell<Map>>;

#[derive(Clone)]
struct Position {
    map: MapPtr,
    point: Point,
}

impl Position {
    fn distance(&self) -> u32 {
        (self.point - self.map.borrow().end).square_length() as u32
    }

    fn successors(&self) -> Vec<(Position, u32)> {
        let map_ptr = self.map.clone();
        let map = self.map.borrow();
        let element = map.get_element(&self.point);
        //println!("from p = {:?} e = {:?}", self.point, element);
        let mut suc = vec![];
        for d in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Some(p) = d.in_direction(self.point, &map.bounds) {
                let new_element = map.get_element(&p);
                //println!("p = {:?} e = {:?}", p, new_element);
                if element.is_legal_from(&new_element) {
                    let pos = Position {
                        map: map_ptr.clone(),
                        point: p,
                    };
                    let dis = pos.distance();
                    suc.push((
                        Position {
                            map: map_ptr.clone(),
                            point: p,
                        },
                        dis,
                    ));
                }
            }
        }
        // for s in &suc {
        //     println!("p = {:?}", s.0.point);
        // }
        suc
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.point == other.point
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.point.hash(hasher)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("x", &self.point.x)
            .field("y", &self.point.y)
            .finish()
    }
}

fn parse(s: &str) -> Map {
    let data: Vec<Vec<Element>> = s
        .lines()
        .map(|l| l.chars().map(Element::from).collect())
        .collect();

    let dimensions: Size = size2(data[0].len() as isize, data.len() as isize);
    let mut start = None;
    let mut end = None;
    for x in 0..dimensions.width {
        for y in 0..dimensions.height {
            let p = point2(x, y);
            let element = data[y as usize][x as usize];
            match element {
                Element::Start => start = Some(p),
                Element::End => end = Some(p),
                _ => (),
            }
        }
    }
    Map {
        bounds: Rect::from_size(dimensions),
        data,
        start: start.unwrap(),
        end: end.unwrap(),
    }
}

fn find_path(map: MapPtr) -> Vec<Position> {
    let start = map.borrow().start;
    let end = map.borrow().end;

    println!("start = {:?}", start);
    println!("end = {:?}", end);

    let position = Position {
        map,
        point: start,
    };
    let (result, _) = astar_bag_collect(
        &position,
        |p| p.successors(),
        |p| p.distance(),
        |p| p.point == end,
    )
    .unwrap();
    for r in &result {
        println!("result len = {}", r.len());
    }
    result[0].clone()
}

fn main() {
    let map = Rc::new(RefCell::new(parse(DATA)));
    let result = find_path(map.clone());
    println!("{}", map.borrow().render_result(&result, DATA));
    println!("fewest steps = {}", result.len() - 1);
}

#[cfg(test)]
mod test {
    use super::*;
    use euclid::{point2, size2};

    const SAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    const RESULT: &str = r#"v..v<<<<
>v.vv<<^
.>vv>E^^
..v>>>^^
..>>>>>^"#;

    #[test]
    fn test_parse() {
        let map = parse(SAMPLE);
        assert_eq!(map.bounds, Rect::from_size(size2(8, 5)));
        assert_eq!(map.start, point2(0, 0));
        assert_eq!(map.end, point2(5, 2));
    }

    #[test]
    fn test_part1() {
        let map = parse(SAMPLE);

        let map = Rc::new(RefCell::new(map));

        let result = find_path(map.clone());

        println!("result = {:?}", result);
        let s = map.borrow().render_result(&result);
        println!("{}", s);
        assert_eq!(result.len() - 1, 31);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _ = parse(SAMPLE);
        todo!();
    }
}
