use anyhow::Error;
use euclid::{point3, vec3};
use pathfinding::prelude::*;
use std::collections::HashSet;
use structopt::StructOpt;

type Coord = i64;
type Point = euclid::default::Point3D<Coord>;
type Box3D = euclid::default::Box3D<Coord>;

type PointSet = HashSet<Point>;

const DATA: &str = include_str!("../../data/day18.txt");
const SAMPLE: &str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

fn parse_point(s: &str) -> Point {
    let parts: Vec<Coord> = s
        .split(',')
        .map(str::parse::<Coord>)
        .map(Result::unwrap_or_default)
        .collect();
    assert_eq!(parts.len(), 3);
    point3(parts[0], parts[1], parts[2])
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day18", about = "Boiling Boulders")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn count_neighbors(p: &Point, points: &PointSet) -> usize {
    let mut neighbors = 0;
    for x in [-1, 1] {
        let new_p = *p + vec3(x, 0, 0);
        if points.contains(&new_p) {
            neighbors += 1;
        }
    }
    for y in [-1, 1] {
        let new_p = *p + vec3(0, y, 0);
        if points.contains(&new_p) {
            neighbors += 1;
        }
    }
    for z in [-1, 1] {
        let new_p = *p + vec3(0, 0, z);
        if points.contains(&new_p) {
            neighbors += 1;
        }
    }

    neighbors
}

fn taxicab_distance(p: &Point, q: &Point) -> Coord {
    let p2 = (*p - *q).abs();
    p2.x + p2.y + p.z
}

fn successors(pt: &Point, end: &Point, search_box: &Box3D, points: &PointSet) -> Vec<(Point, usize)> {
    let deltas = [
        vec3(-1, 0, 0),
        vec3(1, 0, 0),
        vec3(0, -1, 0),
        vec3(0, 1, 0),
        vec3(0, 0, -1),
        vec3(0, 0, 1),
    ];
    let s = deltas
        .iter()
        .map(|v| *pt + *v)
        .filter_map(|pt| {
            (search_box.contains(pt) && (pt == *end || points.contains(&pt) == false)).then_some(pt)
        })
        .map(|pt| (pt, 1))
        .collect();
    // dbg!(&s);
    s
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {}

fn has_path(start: Point, end: &Point, search_box: &Box3D, points: &PointSet) -> bool {
    astar(
        &start,
        |p| successors(p, end, search_box, points),
        |p| taxicab_distance(p, &end) as usize,
        |p| *p == *end,
    )
    .is_some()
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let points: PointSet = if opt.puzzle_input { DATA } else { SAMPLE }
        .lines()
        .map(parse_point)
        .collect();

    let mut faces: usize = 0;

    for p in points.iter() {
        faces += 6 - count_neighbors(p, &points);
    }

    println!("faces = {faces}");

    let bbox = Box3D::from_points(points.iter());
    let search_box = bbox.inflate(2, 2, 2);
    println!("bbox = {bbox:?}");
    let mut bubbles = vec![];
    for z in bbox.min.z..bbox.max.z {
        for y in bbox.min.y..bbox.max.y {
            for x in bbox.min.x..bbox.max.x {
                let p = point3(x, y, z);
                if !points.contains(&p) && count_neighbors(&p, &points) <= 6 {
                    bubbles.push(p);
                }
            }
        }
    }

    println!("bubbles = {}", bubbles.len());

    let start = point3(-1, -1, -1);
	bubbles.retain(|b| !has_path(start, b, &search_box, &points));
	
	let mut points2 = points.clone();
	points2.extend(bubbles.iter());

    println!("bubbles = {}", bubbles.len());

	faces = 0;
    for p in points2.iter() {
        faces += 6 - count_neighbors(p, &points2);
    }
	
    println!("faces = {faces}");
	

    Ok(())
}
