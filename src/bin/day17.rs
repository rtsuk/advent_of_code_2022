use anyhow::Error;
use console::Term;
use euclid::{point2, vec2};
use std::collections::HashSet;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day17.txt");
const SAMPLE: &str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

#[derive(Debug, StructOpt)]
#[structopt(name = "day17", about = "Pyroclastic Flow")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    /// Run step by step
    #[structopt(short, long)]
    interactive: bool,
}

#[derive(Debug, Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("unknown jet"),
        }
    }
}

impl From<&Jet> for Vector {
    fn from(j: &Jet) -> Self {
        match j {
            Jet::Left => vec2(-1, 0),
            Jet::Right => vec2(1, 0),
        }
    }
}

type Jets = Vec<Jet>;

type Point = euclid::default::Point2D<isize>;
type Vector = euclid::default::Vector2D<isize>;
type Box = euclid::default::Box2D<isize>;

type BlockSet = HashSet<Point>;

const MAX_X: isize = 7;

fn block_collides_with_wall(p: &&Point) -> bool {
    p.x < 0 || p.x >= MAX_X
}

fn block_collides_with_floor(p: &&Point) -> bool {
    p.y < 0
}

#[derive(Debug)]
struct Shape {
    blocks: Vec<Point>,
    name: char,
}

// impl Debug for Shape {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }

impl Shape {
    fn horiz() -> Shape {
        let blocks = (0..4).map(|x| point2(x, 0)).collect();
        Self { blocks, name: '-' }
    }

    fn plus() -> Shape {
        let blocks = [
            point2(1, 0),
            point2(0, 1),
            point2(1, 1),
            point2(2, 1),
            point2(1, 2),
        ]
        .to_vec();
        Self { blocks, name: '+' }
    }

    fn inverted_l() -> Shape {
        let blocks = [
            point2(2, 2),
            point2(2, 1),
            point2(0, 0),
            point2(1, 0),
            point2(2, 0),
        ]
        .to_vec();
        Self {
            blocks, name: '⅃'
        }
    }

    fn vertical() -> Shape {
        let blocks = (0..4).map(|y| point2(0, y)).collect();
        Self { blocks, name: '|' }
    }

    fn block() -> Shape {
        let blocks = [point2(0, 0), point2(1, 0), point2(0, 1), point2(1, 1)].to_vec();
        Self {
            blocks, name: '▀'
        }
    }

    fn shape_for(index: usize) -> Self {
        match index % 5 {
            0 => Self::horiz(),
            1 => Self::plus(),
            2 => Self::inverted_l(),
            3 => Self::vertical(),
            4 => Self::block(),
            _ => unreachable!(),
        }
    }

    fn translate(&self, v: Vector) -> Shape {
        let blocks = self.blocks.iter().map(|p| *p + v).collect();
        Self {
            blocks,
            name: self.name,
        }
    }

    fn collides_with_wall(&self) -> bool {
        self.blocks.iter().find(block_collides_with_wall) != None
    }

    fn collides_with_floor(&self) -> bool {
        self.blocks.iter().find(block_collides_with_floor) != None
    }

    fn collides_with(&self, block_set: &BlockSet) -> bool {
        self.blocks.iter().find(|p| block_set.contains(p)) != None
    }

    fn bounding_box(&self) -> Box {
        Box::from_points(self.blocks.iter())
    }

    fn shape_set(&self) -> BlockSet {
        self.blocks.iter().copied().collect()
    }
}

fn parse(s: &str) -> Jets {
    s.chars().map(Jet::from).collect::<Vec<Jet>>()
}

fn render(block_set: &BlockSet, shape_set: &BlockSet) {
    let total_box = Box::from_points(block_set.iter().chain(shape_set.iter()));
    println!("total_box = {:?}", total_box);

    for y in (0..(total_box.max.y + 1)).rev() {
        let s = (0..MAX_X)
            .map(|x| {
                let p = point2(x, y);
                if block_set.contains(&p) {
                    '#'
                } else if shape_set.contains(&p) {
                    '@'
                } else {
                    '.'
                }
            })
            .collect::<String>();
        println!("|{}|", s);
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let term = Term::stdout();

    let bursts = parse(if !opt.puzzle_input { SAMPLE } else { DATA });
    let bursts_len = bursts.len();

    let mut starting_y = 0;
    let mut block_set: BlockSet = HashSet::new();
    let mut jet_index = 0;
    for i in 0..2023 {
        let mut shape = Shape::shape_for(i);
        let v = vec2(2, starting_y + 3);
        shape = shape.translate(v);
        if opt.interactive {
            let shape_set = shape.shape_set();
            render(&block_set, &shape_set);
        }
        loop {
            if opt.interactive {
                let _ = term.read_char()?;
            }

            let jet = bursts[jet_index % bursts_len];
            jet_index += 1;
            let v = Vector::from(&jet);
            let new_shape = shape.translate(v);
            if !new_shape.collides_with_wall() && !new_shape.collides_with(&block_set) {
                shape = new_shape;
            }
            if opt.interactive {
                let shape_set = shape.shape_set();
                render(&block_set, &shape_set);
                let _res = term.read_char()?;
            }

            let new_shape = shape.translate(vec2(0, -1));
            if new_shape.collides_with_floor() || new_shape.collides_with(&block_set) {
                block_set.extend(shape.blocks.iter());
                let bbox = shape.bounding_box();
                starting_y = starting_y.max(bbox.max.y + 1);
                break;
            } else {
                shape = new_shape;
            }
            if opt.interactive {
                let shape_set = shape.shape_set();
                render(&block_set, &shape_set);
            }
        }
    }

    let bbox = Box::from_points(block_set.iter());

    render(&block_set, &HashSet::new());

    println!("bbox = {:?}", bbox);

    // 2568 is too low
    // 2894 is too low
	// 3171 is too low

    Ok(())
}
