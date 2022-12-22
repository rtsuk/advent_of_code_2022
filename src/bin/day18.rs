use anyhow::Error;
use euclid::{point3, vec3};
use std::collections::HashSet;
use structopt::StructOpt;

type Point = euclid::default::Point3D<isize>;
type Box3D = euclid::default::Box3D<isize>;

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
    let parts: Vec<isize> = s
        .split(',')
        .map(str::parse::<isize>)
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
    println!("bbox = {bbox:?}");
    let mut bubbles = 0;
    for z in bbox.min.z..bbox.max.z {
        for y in bbox.min.y..bbox.max.y {
            for x in bbox.min.x..bbox.max.x {
                let p = point3(x, y, z);
                if count_neighbors(&p, &points) == 6 {
                    println!("bubble = {p:?}");
                    bubbles += 1;
                }
            }
        }
    }
    println!("bubbles = {bubbles}");
    println!("faces = {}", faces - bubbles * 6);

    Ok(())
}
