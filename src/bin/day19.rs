use anyhow::Error;
use enum_iterator::{all, Sequence};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::BTreeSet,
    ops::{Add, AddAssign, Mul, Range, Sub},
};
use structopt::StructOpt;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Sequence)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type ResourceCount = usize;

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

    #[structopt(long, default_value = "24")]
    time_limit: usize,

    #[structopt(long, default_value = "2000")]
    blueprint_limit: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Ord, Hash, Eq)]
struct Resources {
    geode: ResourceCount,
    obsidian: ResourceCount,
    clay: ResourceCount,
    ore: ResourceCount,
}

impl Resources {
    fn contains(&self, other: &Resources) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }

    fn total_resources(&self) -> ResourceCount {
        self.ore + self.clay + self.obsidian + self.geode
    }
}

impl Mul<ResourceCount> for Resources {
    type Output = Self;

    fn mul(self, rhs: ResourceCount) -> Self {
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

#[derive(Debug, Default, Hash, PartialEq, Eq)]
struct RobotDelivery {
    time: usize,
    robots: Robots,
}

type StateSet = BTreeSet<State>;

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct State {
    robots: Robots,
    resources: Resources,
}

impl State {
    fn starting() -> Self {
        Self {
            robots: Robots {
                ore: 1,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn with_order(&self, bp: &Blueprint, _time: usize, robot_order: Robots) -> Self {
        let mut resources = self.resources - bp.build_cost(&robot_order);
        resources += resources_made(&self.robots);
        let robots = self.robots + robot_order;

        Self { robots, resources }
    }

    fn step(&self, bp: &Blueprint, time: usize, _limit: usize) -> StateSet {
        let orders = order_permutation_s(&self.resources, &self.robots, bp);

        orders
            .into_iter()
            .map(|o| self.with_order(bp, time, o))
            .collect()
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
        }
    }

    fn robot_cost(&self, resource_type: ResourceType) -> Resources {
        match resource_type {
            ResourceType::Ore => self.ore_robot,
            ResourceType::Clay => self.clay_robot,
            ResourceType::Obsidian => self.obsidian_robot,
            ResourceType::Geode => self.geode_robot,
        }
    }

    fn build_cost(&self, robots: &Robots) -> Resources {
        let mut cost = Default::default();
        for rt in all::<ResourceType>() {
            if robots.contains(rt) {
                cost += self.robot_cost(rt);
            }
        }
        cost
    }
}

fn parse(s: &str) -> Vec<Blueprint> {
    let re = Regex::new(
        r#"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.
"#,
    ).expect("re");

    re.captures_iter(s).map(Blueprint::new).collect()
}

#[derive(Debug, Default, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
struct Robots {
    geode: ResourceCount,
    obsidian: ResourceCount,
    clay: ResourceCount,
    ore: ResourceCount,
}

impl Robots {
    fn contains(&self, resource_type: ResourceType) -> bool {
        match resource_type {
            ResourceType::Ore => self.ore > 0,
            ResourceType::Clay => self.clay > 0,
            ResourceType::Obsidian => self.obsidian > 0,
            ResourceType::Geode => self.geode > 0,
        }
    }
}

impl Add for Robots {
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

fn order_permutation_s(
    resources: &Resources,
    _robots: &Robots,
    blueprint: &Blueprint,
) -> Vec<Robots> {
    let possible_builds = vec![
        Robots::default(),
        Robots {
            geode: 1,
            ..Robots::default()
        },
        Robots {
            ore: 1,
            ..Robots::default()
        },
        Robots {
            clay: 1,
            ..Robots::default()
        },
        Robots {
            obsidian: 1,
            ..Robots::default()
        },
    ];
    let mut p = vec![];
    for r in possible_builds.iter() {
        let cost = blueprint.build_cost(r);
        if resources.contains(&cost) {
            p.push(*r);
        }
    }

    p
}

#[allow(unused)]
fn order_permutation(resources: &Resources, robots: &Robots, blueprint: &Blueprint) -> Vec<Robots> {
    const ZERO_OR_ONE: Range<ResourceCount> = 0..2;
    let mut p = vec![];
    let max_clay = if robots.clay < blueprint.obsidian_robot.clay {
        2
    } else {
        1
    };
    let max_ore = if robots.ore
        < (blueprint.obsidian_robot.ore
            + blueprint.geode_robot.ore
            + blueprint.clay_robot.ore
            + blueprint.ore_robot.ore)
    {
        2
    } else {
        1
    };
    let max_obsidian = if robots.obsidian < blueprint.geode_robot.obsidian {
        2
    } else {
        1
    };
    for geode in ZERO_OR_ONE {
        for obsidian in 0..max_obsidian {
            for clay in 0..max_clay {
                for ore in 0..max_ore {
                    let robots = Robots {
                        ore,
                        clay,
                        obsidian,
                        geode,
                    };
                    let cost = blueprint.build_cost(&Robots {
                        ore,
                        clay,
                        obsidian,
                        geode,
                    });
                    if resources.contains(&cost) {
                        p.push(robots);
                    }
                }
            }
        }
    }
    p
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

    let mut quality_level = 0;
    let mut total = 1;
    let blueprint_limit = opt.blueprint_limit.min(blueprints.len());
    for bp in &blueprints[0..blueprint_limit] {
        let mut states: StateSet = StateSet::new();
        states.insert(State::starting());

        for time in 1..=opt.time_limit {
            println!("### time = {time} state count = {}", states.len());
            let new_states: StateSet = states
                .par_iter()
                .flat_map(|state| state.step(bp, time, opt.time_limit))
                .collect();

            let mut new_state_pared = StateSet::new();
            for (_key, group) in &new_states.iter().group_by(|s| s.robots) {
                let mut state_group = group.collect::<Vec<_>>();
                state_group.sort_by_key(|s| s.resources.total_resources());
                state_group.reverse();
                for state in &state_group[0..10.min(state_group.len())] {
                    new_state_pared.insert(**state);
                }
            }
            states = new_state_pared;
        }

        println!("done");

        let mut state_list: Vec<_> = states.into_iter().collect();

        state_list.sort_by_key(|s| s.resources);
        state_list.reverse();
        let geodes = state_list[0].resources.geode;
        println!("state = {:#?}", &state_list[0]);
        quality_level += bp.id * geodes;
        total *= geodes;
    }
    println!("quality_level = {quality_level}");
    println!("total = {total}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let bps = parse(SAMPLE);
        dbg!(&bps);
        assert_eq!(bps.len(), 2);
    }

    #[test]
    fn test_order_permutation() {
        let bps = parse(SAMPLE);
        let bp0 = &bps[0];

        let r = Resources::default();
        let robots = Robots::default();
        let orders = order_permutation_s(&r, &robots, bp0);

        assert_eq!(orders.len(), 1);

        let r = Resources {
            ore: 4,
            ..Resources::default()
        };

        let orders = order_permutation_s(&r, &robots, bp0);

        dbg!(&orders);
        assert_eq!(orders.len(), 3);

        let r = Resources {
            ore: 8,
            clay: 14,
            ..Resources::default()
        };

        let orders = order_permutation_s(&r, &robots, bp0);

        assert_eq!(orders.len(), 4);
        let r = Resources {
            ore: 4,
            clay: 15,
            ..Resources::default()
        };

        let orders = order_permutation_s(&r, &robots, bp0);

        dbg!(&orders);

        assert_eq!(orders.len(), 4);
    }

    #[test]
    fn test_time_10() {
        let bps = parse(SAMPLE);
        let bp0 = &bps[0];

        println!("bp = {:#?}", bp0);

        let state = State {
            robots: Robots {
                ore: 1,
                clay: 3,
                ..Robots::default()
            },
            resources: Resources {
                ore: 4,
                clay: 15,
                ..Resources::default()
            },
        };

        let expected_state = State {
            robots: Robots {
                ore: 1,
                clay: 3,
                obsidian: 1,
                ..Robots::default()
            },
            resources: Resources {
                ore: 2,
                clay: 4,
                ..Resources::default()
            },
        };
        let new_state = state.with_order(
            bp0,
            10,
            Robots {
                obsidian: 1,
                ..Robots::default()
            },
        );

        assert_eq!(new_state, expected_state);
    }

    #[test]
    fn test_solve() {
        let bps = parse(SAMPLE);
        let bp0 = &bps[0];

        println!("bp = {:#?}", bp0);

        let expected_states: &[(usize, State)] = &[
            (0, State::starting()),
            (
                1,
                State {
                    robots: Robots {
                        ore: 1,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 1,
                        ..Resources::default()
                    },
                },
            ),
            (
                2,
                State {
                    robots: Robots {
                        ore: 1,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 2,
                        ..Resources::default()
                    },
                },
            ),
            (
                3,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 1,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 1,
                        clay: 0,
                        ..Resources::default()
                    },
                },
            ),
            (
                4,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 1,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 2,
                        clay: 1,
                        ..Resources::default()
                    },
                },
            ),
            (
                5,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 2,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 1,
                        clay: 2,
                        ..Resources::default()
                    },
                },
            ),
            (
                6,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 2,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 2,
                        clay: 4,
                        ..Resources::default()
                    },
                },
            ),
            (
                7,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 3,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 1,
                        clay: 6,
                        ..Resources::default()
                    },
                },
            ),
            (
                8,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 3,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 2,
                        clay: 9,
                        ..Resources::default()
                    },
                },
            ),
            (
                9,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 3,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 3,
                        clay: 12,
                        ..Resources::default()
                    },
                },
            ),
            (
                10,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 3,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 4,
                        clay: 15,
                        ..Resources::default()
                    },
                },
            ),
            (
                11,
                State {
                    robots: Robots {
                        ore: 1,
                        clay: 3,
                        obsidian: 1,
                        ..Robots::default()
                    },
                    resources: Resources {
                        ore: 2,
                        clay: 4,
                        ..Resources::default()
                    },
                },
            ),
        ];

        let mut states: StateSet = StateSet::new();
        states.insert(State::starting());

        for (i, expected_state) in expected_states.iter().enumerate() {
            let time = i + 1;
            if !states.contains(&expected_state.1) {
                println!("### time = {time}");
                let mut state_list: Vec<_> = states.into_iter().collect();
                state_list.sort_by_key(|s| s.resources);
                println!("### states = {:#?}", state_list);
                println!("### expected_state = {:#?}", expected_state);
                panic!();
            }
            let new_states: StateSet = states
                .iter()
                .flat_map(|state| state.step(bp0, time, 24))
                .collect();
            states = new_states;
        }

        let mut state_list: Vec<_> = states.into_iter().collect();

        state_list.sort_by_key(|s| s.resources);
        state_list.reverse();

        println!("states = {:#?}", &state_list[..4.min(state_list.len())]);
    }
}
