#![allow(unused)]
use anyhow::Error;
use internment::Intern;
use itertools::Itertools;
use once_cell::sync::Lazy;
use pathfinding::prelude::*;
use petgraph::{
    dot::{Config, Dot},
    graphmap::UnGraphMap,
    visit::{EdgeRef, NodeRef},
};
use regex::Regex;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    sync::Mutex,
};
use structopt::StructOpt;

type Coord = u8;
type RoomId = Intern<String>;
type OpenValves = HashSet<RoomId>;

const DATA: &str = include_str!("../../data/day16.txt");
const EXAMPLE_SOLUTION: &str = include_str!("../../data/day16_example.txt");
const SAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

#[derive(Debug, Clone)]
struct Room {
    room_id: RoomId,
    flow: usize,
    tunnels: Vec<RoomId>,
}

impl Room {
    fn new(captures: regex::Captures) -> Self {
        let room_id = Intern::new(captures[1].to_string());
        Self {
            room_id,
            flow: captures[2].parse::<usize>().expect("usize"),
            tunnels: captures[3]
                .split(", ")
                .map(|s| Intern::new(s.to_string()))
                .collect(),
        }
    }
}

type RoomMap = HashMap<RoomId, Room>;
type FlowGraph = UnGraphMap<RoomId, String>;

#[derive(Debug, PartialEq)]
enum Action {
    Move(RoomId),
    Open,
    Idle,
}

#[derive(Default, Debug, Clone)]
struct Volcano {
    rooms: RoomMap,
    graph: FlowGraph,
}

fn successors(point: &RoomId, graph: &FlowGraph) -> Vec<RoomId> {
    graph.neighbors(*point).collect()
}

impl Volcano {
    fn new(rooms: RoomMap) -> Self {
        let graph = Self::make_graph(&rooms);
        Self { rooms, graph }
    }

    fn make_graph(rooms: &RoomMap) -> FlowGraph {
        let edges: Vec<_> = rooms
            .values()
            .flat_map(|r| {
                r.tunnels
                    .iter()
                    .map(|t| (r.room_id, *t))
                    .collect::<Vec<_>>()
            })
            .collect();

        FlowGraph::from_edges(&edges)
    }

    fn path_between(&self, start: &RoomId, end: &RoomId) -> Vec<RoomId> {
        let graph = self.graph.clone();
        let path = bfs(start, |p| successors(p, &graph), |p| p == end).unwrap();
        path[1..].to_vec()
    }

    fn path_between_str(&self, start: &str, end: &str) -> Vec<RoomId> {
        let start = RoomId::new(start.to_string());
        let end = RoomId::new(end.to_string());
        self.path_between(&start, &end)
    }

    fn rooms_with_valves(&self) -> Vec<RoomId> {
        self.rooms
            .values()
            .filter_map(|r| (r.flow > 0).then_some(r.room_id))
            .collect()
    }

    fn current_flow(&self, open_valves: &OpenValves) -> usize {
        open_valves
            .iter()
            .map(|room_id| self.rooms.get(room_id).expect("room").flow)
            .sum()
    }

    fn actions(&self, path: &[RoomId]) -> Vec<Action> {
        let mut actions = vec![];
        for room_id in path.iter() {
            actions.push(Action::Move(*room_id));
        }
        actions.push(Action::Open);
        actions
    }
}

fn to_string(path: &[&RoomId]) -> String {
    path.iter()
        .map(|r| r.as_ref().to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn to_path(path: &[&str]) -> Vec<RoomId> {
    path.iter()
        .map(|r| Intern::new(r.to_string()))
        .collect::<Vec<RoomId>>()
}

fn to_ref_path<'a>(path: &'a [RoomId]) -> Vec<&'a RoomId> {
    path.iter().map(|r| r).collect::<Vec<&'a RoomId>>()
}

fn solve(volcano: &Volcano, start: &RoomId, path: &[&RoomId], limit: usize) -> usize {
    let mut total_pressure = 0;
    let mut open_valves = OpenValves::default();
    let mut player_location = *start;
    let mut time = 1;
    for next_destination in path {
        let path_to_next_destination = volcano.path_between(&player_location, next_destination);
        let actions = volcano.actions(path_to_next_destination.as_slice());
        for action in actions {
            let current_flow = volcano.current_flow(&open_valves);
            total_pressure += current_flow;
            match &action {
                Action::Move(t) => {
                    player_location = *t;
                }

                Action::Open => {
                    open_valves.insert(player_location);
                }

                Action::Idle => (),
            }
            time += 1;

            if time > limit {
                // println!("#### solving {:?}", to_string(path));
                // println!("### ran out of time");
                return total_pressure;
            }
        }
    }
    while time <= limit {
        let current_flow = volcano.current_flow(&open_valves);
        total_pressure += current_flow;
        time += 1;
    }

    total_pressure
}

fn parse(s: &str) -> Volcano {
    let re = Regex::new(
        r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels* leads* to valves* ([A-Z, ]+)",
    )
    .expect("re");

    let rooms = re
        .captures_iter(s)
        .map(Room::new)
        .map(|r| (r.room_id, r))
        .collect();

    Volcano::new(rooms)
}

#[derive(Debug)]
struct ExampleStep {
    time: usize,
    open_valves: Vec<String>,
    pressure: usize,
    action: Action,
}

impl From<&str> for ExampleStep {
    fn from(s: &str) -> Self {
        let parts = s.lines().map(str::trim).collect::<Vec<_>>();
        let re = Regex::new(r"== Minute (\d+) ==").expect("regex");
        let time = re.captures(parts[0]).expect("captures")[1]
            .parse::<usize>()
            .expect("usize");
        let valve_info = parts[1];

        let mut open_valves: Vec<String> = vec![];
        let mut pressure;
        let mut action = Action::Idle;

        match valve_info {
            "No valves are open." => {
                pressure = 0;
                open_valves = vec![];
            }
            _ => {
                let re = Regex::new(r"[A-Z][A-Z]").expect("regex");
                open_valves = re
                    .captures_iter(valve_info)
                    .map(|cap| cap[0].to_string())
                    .collect();
                let re = Regex::new(r"releasing (\d+) pressure").expect("regex");
                pressure = re.captures(valve_info).expect("captures")[1]
                    .parse::<usize>()
                    .expect("usize");
            }
        }

        if parts.len() > 2 {
            let re = Regex::new(r"You ([a-z]+).*valve ([A-Z][A-Z]).").expect("re");
            let captures = re.captures(parts[2]).expect("captures");
            action = match &captures[1] {
                "move" => Action::Move(RoomId::new(captures[2].to_string())),
                "open" => Action::Open,
                _ => Action::Idle,
            };
        }

        Self {
            time,
            open_valves,
            pressure,
            action,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day15", about = "Beacon Exclusion Zone")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,

    /// Output graph drawing instructions
    #[structopt(short, long)]
    graph: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let mut volcano = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    if opt.graph {
        println!(
            "{:?}",
            Dot::with_attr_getters(
                &volcano.graph,
                &[Config::NodeNoLabel, Config::EdgeNoLabel],
                &|_, er| format!("label = \"{}\"", er.weight()),
                &|_, nr| format!("label = \"{}\"", nr.weight()),
            ),
        );
    } else {
        let rooms = volcano.rooms_with_valves();
        println!("{} rooms, {:?}", rooms.len(), rooms);

        let start_room = RoomId::new("AA".to_string());

        let mut solutions: Vec<_> = rooms
            .iter()
            .permutations(rooms.len().min(6))
            .map(|path| (solve(&volcano, &start_room, path.as_slice(), 30), path.clone()))
            .collect();

        solutions.sort_by_key(|s| s.0);

        solutions.reverse();

        println!("total pressure = {}", solutions[0].0);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let v = parse(SAMPLE);
        dbg!(&v);
        assert_eq!(v.rooms.len(), 10);
    }

    #[test]
    fn test_example_solution() {
        let example_steps: Vec<_> = EXAMPLE_SOLUTION
            .split("\n\n")
            .map(ExampleStep::from)
            .collect();

        assert_eq!(example_steps.len(), 30);
        assert_eq!(
            example_steps[0].action,
            Action::Move(RoomId::new("DD".to_string()))
        );
        assert_eq!(example_steps[0].pressure, 0);
        assert_eq!(example_steps[0].open_valves.len(), 0);

        let middle_step = &example_steps[17];
        assert_eq!(
            middle_step.action,
            Action::Move(RoomId::new("GG".to_string()))
        );
        assert_eq!(middle_step.pressure, 76);
        assert_eq!(middle_step.open_valves.len(), 4);

        let last_step = &example_steps[29];
        assert_eq!(last_step.action, Action::Idle);
        assert_eq!(last_step.pressure, 81);
        assert_eq!(last_step.open_valves.len(), 6);

        let v = parse(SAMPLE);

        let mut total_pressure = 0;

        let mut open_valves = OpenValves::default();
        let mut player_location = RoomId::new("AA".to_string());

        for step in example_steps.iter() {
            println!("doing step {:?}", step);
            let current_flow = v.current_flow(&open_valves);
            total_pressure += current_flow;
            assert_eq!(step.pressure, current_flow);
            match &step.action {
                Action::Move(t) => {
                    player_location = RoomId::new(t.to_string());
                }

                Action::Open => {
                    open_valves.insert(player_location);
                }

                Action::Idle => (),
            }
            dbg!(&player_location);
            dbg!(&open_valves);
        }

        assert_eq!(total_pressure, 1651);
    }

    #[test]
    fn test_volcano() {
        let mut v = parse(SAMPLE);

        let path = v.path_between_str("AA", "HH");
        assert_eq!(path.len(), 5);

        dbg!(&path);
    }

    #[test]
    fn test_permute_solve() {
        let v = parse(SAMPLE);
        let start_room = RoomId::new("AA".to_string());

        let rooms = v.rooms_with_valves();

        let one_path = to_path(&["DD", "BB", "JJ", "HH", "EE", "CC"]);
        let one_solution = solve(&v, &start_room, &to_ref_path(one_path.as_slice()), 30);
        assert_eq!(one_solution, 1651);

        let mut solutions: Vec<_> = rooms
            .iter()
            .permutations(rooms.len())
            .map(|path| (solve(&v, &start_room, path.as_slice(), 30), path.clone()))
            .collect();

        solutions.sort_by_key(|s| s.0);

        solutions.reverse();

        dbg!(&solutions[0..10]);

        assert_eq!(solutions[0].1, to_ref_path(one_path.as_slice()));
    }
}
