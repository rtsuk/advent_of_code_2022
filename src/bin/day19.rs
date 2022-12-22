use anyhow::Error;
use regex::Regex;
use std::ops::{Add, AddAssign, Mul, Sub};
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day19.txt");
const SAMPLE: &str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

#[derive(Debug, StructOpt)]
#[structopt(name = "day19", about = "Not Enough Minerals")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
            geode: self.geode * rhs,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore.checked_sub(other.ore).unwrap(),
            clay: self.clay.checked_sub(other.clay).unwrap(),
            obsidian: self.obsidian.checked_sub(other.obsidian).unwrap(),
            geode: self.geode.checked_sub(other.geode).unwrap(),
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore.checked_add(other.ore).unwrap(),
            clay: self.clay.checked_add(other.clay).unwrap(),
            obsidian: self.obsidian.checked_add(other.obsidian).unwrap(),
            geode: self.geode.checked_add(other.geode).unwrap(),
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore.checked_add(other.ore).unwrap(),
            clay: self.clay.checked_add(other.clay).unwrap(),
            obsidian: self.obsidian.checked_add(other.obsidian).unwrap(),
            geode: self.geode.checked_add(other.geode).unwrap(),
        };
    }
}

#[derive(Debug, Default)]
struct Blueprint {
    id: usize,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl Blueprint {
    fn new(parts: regex::Captures) -> Self {
        Self {
            id: parts[1].parse().expect("id"),
            ore_robot: Resources {
                ore: parts[2].parse().unwrap(),
                ..Resources::default()
            },
            clay_robot: Resources {
                ore: parts[3].parse().unwrap(),
                ..Resources::default()
            },
            obsidian_robot: Resources {
                ore: parts[4].parse().unwrap(),
                clay: parts[5].parse().unwrap(),
                ..Resources::default()
            },
            geode_robot: Resources {
                ore: parts[6].parse().unwrap(),
                obsidian: parts[7].parse().unwrap(),
                ..Resources::default()
            },
            ..Self::default()
        }
    }
}

fn parse(s: &str) -> Vec<Blueprint> {
    let re = Regex::new(
        r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.
"#,
    ).expect("re");

    re.captures_iter(s).map(|c| Blueprint::new(c)).collect()
}

#[derive(Debug, Default)]
struct Robots {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Add for Robots {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            ore: self.ore.checked_add(other.ore).unwrap(),
            clay: self.clay.checked_add(other.clay).unwrap(),
            obsidian: self.obsidian.checked_add(other.obsidian).unwrap(),
            geode: self.geode.checked_add(other.obsidian).unwrap(),
        }
    }
}

impl AddAssign for Robots {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            ore: self.ore.checked_add(other.ore).unwrap(),
            clay: self.clay.checked_add(other.clay).unwrap(),
            obsidian: self.obsidian.checked_add(other.obsidian).unwrap(),
            geode: self.geode.checked_add(other.geode).unwrap(),
        };
    }
}

fn legal_order(resources: Resources, blueprint: &Blueprint) -> (Robots, Resources) {
    let mut robots = Robots::default();

    let max_geode = (resources.obsidian / blueprint.geode_robot.obsidian)
        .min(resources.obsidian / blueprint.geode_robot.obsidian);
    let resources = resources - blueprint.geode_robot * max_geode;
    robots.geode = max_geode;

    let max_obsidian = (resources.ore / blueprint.obsidian_robot.ore)
        .min(resources.clay / blueprint.obsidian_robot.clay);
    let resources = resources - blueprint.obsidian_robot * max_obsidian;
    robots.obsidian = max_obsidian;

    let max_clay = resources.ore / blueprint.clay_robot.ore;
    let resources = resources - blueprint.clay_robot * max_clay;
    robots.clay = max_clay;

    let max_ore = resources.ore / blueprint.ore_robot.ore;
    let resources = resources - blueprint.ore_robot * max_ore;
    robots.ore = max_ore;

    (robots, resources)
}

fn resources_made(robots: &Robots) -> Resources {
    Resources {
        ore: robots.ore,
        clay: robots.clay,
        obsidian: robots.obsidian,
        geode: robots.geode,
    }
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let blueprints = parse(if opt.puzzle_input { DATA } else { SAMPLE });


    let bp = &blueprints[0];

    let mut robots = Robots {
        ore: 1,
        ..Default::default()
    };
    let mut resources = Resources::default();

    for time in 1..=24 {
		println!("#### time {time}: robots = {robots:#?} ");
        resources += resources_made(&robots);
		println!("resources = {resources:#?} ");
        let (new_robots, remainder) = legal_order(resources, &bp);
		println!("new_robots = {new_robots:#?}");
		println!("remainder = {remainder:#?}");
        robots += new_robots;
        resources = remainder;
		
    }

    Ok(())
}
