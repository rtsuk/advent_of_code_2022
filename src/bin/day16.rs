use anyhow::Error;
use internment::Intern;
use itertools::Itertools;
use pathfinding::prelude::*;
use petgraph::{
    dot::{Config, Dot},
    graphmap::UnGraphMap,
    visit::{EdgeRef, NodeRef},
};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Debug, Display},
};
use structopt::StructOpt;

type OpenValves = HashSet<RoomId>;

#[derive(Clone, PartialEq, Hash, Copy, PartialOrd, Ord, Eq)]
struct RoomId(Intern<String>);

impl RoomId {
    fn new(s: &str) -> Self {
        Self(Intern::new(s.to_string()))
    }
}

impl Display for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

impl Debug for RoomId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_ref())
    }
}

const DATA: &str = include_str!("../../data/day16.txt");
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
            room_id: RoomId(room_id),
            flow: captures[2].parse::<usize>().expect("usize"),
            tunnels: captures[3]
                .split(", ")
                .map(|s| RoomId(Intern::new(s.to_string())))
                .collect(),
        }
    }
}

type RoomMap = HashMap<RoomId, Room>;
type FlowGraph = UnGraphMap<RoomId, String>;

#[derive(Debug, PartialEq)]
#[allow(unused)]
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

    fn valued_path_between(
        &self,
        start: &RoomId,
        end: &RoomId,
        limit: usize,
    ) -> (usize, Vec<RoomId>) {
        let path = self.path_between(start, end);
        let len = path.len();
        let flow = self.rooms.get(end).expect("room").flow;
        let value = limit.saturating_sub(len + 1) * flow;
        (value, path)
    }

	#[cfg(test)]
    fn path_between_str(&self, start: &str, end: &str) -> Vec<RoomId> {
        let start = RoomId::new(start);
        let end = RoomId::new(end);
        self.path_between(&start, &end)
    }

    fn rooms_with_valves(&self) -> Vec<RoomId> {
        self.rooms
            .values()
            .filter_map(|r| (r.flow > 0).then_some(r.room_id))
            .collect()
    }

    fn remaining_closed_valves(&self, open_valves: &OpenValves) -> Vec<RoomId> {
        self.rooms
            .values()
            .filter_map(|r| (r.flow > 0 && !open_valves.contains(&r.room_id)).then_some(r.room_id))
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

#[derive(Default, Debug)]
enum Mode {
    Moving(usize, RoomId),
    Opening(RoomId),
    #[default]
    Idle,
}

#[derive(Default, Debug)]
struct Solver {
    path: Vec<RoomId>,
    open_valves: OpenValves,
    current_flow: usize,
    total_pressure: usize,
    mode: Mode,
}

const TIME_LIMIT: usize = 30;

impl Solver {
    fn step(&mut self, _index: usize, time: usize, volcano: &Volcano) -> Option<Vec<Solver>> {
        // println!("#### {index}@{time} step {self:#?}");
        self.total_pressure += self.current_flow;
        match self.mode {
            Mode::Idle => {
                // println!("{index} idle");
                None
            }
            Mode::Moving(mut distance_remaining, target) => {
                distance_remaining -= 1;
                if distance_remaining == 0 {
                    // println!("{index} reached {}", target);
                    self.mode = Mode::Opening(target);
                } else {
                    self.mode = Mode::Moving(distance_remaining, target);
                }
                None
            }
            Mode::Opening(target) => {
                self.path.push(target);
                // println!("{index} opening target {}", target);
                self.open_valves.insert(target);
                self.current_flow = volcano.current_flow(&self.open_valves);
                let remaining_closed_valves = volcano.remaining_closed_valves(&self.open_valves);
                // println!(
                //     "{index} remaining_closed_valves = {:?}",
                //     to_string(remaining_closed_valves.as_slice())
                // );
                if remaining_closed_valves.is_empty() {
                    self.mode = Mode::Idle;
                    None
                } else {
                    let mut paths: Vec<_> = remaining_closed_valves
                        .iter()
                        .map(|r| volcano.valued_path_between(&target, r, TIME_LIMIT - time))
                        .collect();

                    paths.sort_by_key(|p| p.0);
                    paths.reverse();

                    let mut solvers: Vec<_> = paths
                        .iter()
                        .map(|(_value, path)| {
                            let target = *path.iter().last().expect("target");
                            // println!("{index} making new solver for {}", target);
                            Solver {
                                path: self.path.clone(),
                                mode: Mode::Moving(path.len(), target),
                                open_valves: self.open_valves.clone(),
                                current_flow: self.current_flow,
                                total_pressure: self.total_pressure,
                            }
                        })
                        .collect();
                    let mut new_self = solvers.remove(0);
                    std::mem::swap(self, &mut new_self);
                    Some(solvers)
                }
            }
        }
    }
}

fn solver_solve(v: &Volcano) -> usize {
    let start_room = RoomId::new("AA");

    let mut paths: Vec<_> = v
        .rooms_with_valves()
        .iter()
        .map(|r| v.valued_path_between(&start_room, r, TIME_LIMIT))
        .collect();

    paths.sort_by_key(|p| p.0);
    paths.reverse();

    let mut solvers: Vec<_> = paths
        .iter()
        .map(|(_value, path)| Solver {
            mode: Mode::Moving(path.len(), *path.iter().last().expect("target")),
            ..Solver::default()
        })
        .collect();

    for time in 1..=TIME_LIMIT {
		println!("time = {time}");
        let new_solvers: Vec<_> = solvers
            .iter_mut()
            .enumerate()
            .flat_map(|(index, solver)| solver.step(index, time, &v).unwrap_or_default())
            .collect();

        solvers.extend(new_solvers);
    }

    solvers.sort_by_key(|s| s.total_pressure);
    solvers.reverse();

    solvers[0].total_pressure
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

    /// Use permutation
    #[structopt(long)]
    permutation: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let volcano = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

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
        if opt.permutation {
            let rooms = volcano.rooms_with_valves();
            println!("{} rooms, {:?}", rooms.len(), rooms);

            let start_room = RoomId::new("AA");

            let mut solutions: Vec<_> = rooms
                .iter()
                .permutations(rooms.len().min(6))
                .map(|path| {
                    (
                        solve(&volcano, &start_room, path.as_slice(), TIME_LIMIT),
                        path.clone(),
                    )
                })
                .collect();

            solutions.sort_by_key(|s| s.0);

            solutions.reverse();

            println!("total pressure = {}", solutions[0].0);
        } else {
            let total_pressure = solver_solve(&volcano);
            println!("total pressure = {total_pressure}");
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_path(path: &[&str]) -> Vec<RoomId> {
        path.iter().map(|r| RoomId::new(r)).collect::<Vec<RoomId>>()
    }

    fn to_ref_path<'a>(path: &'a [RoomId]) -> Vec<&'a RoomId> {
        path.iter().collect::<Vec<&'a RoomId>>()
    }

    #[derive(Debug)]
    struct ExampleStep {
        #[allow(unused)]
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

            #[allow(unused)]
            let mut open_valves: Vec<String> = vec![];
            let pressure;
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
                    "move" => Action::Move(RoomId::new(&captures[2])),
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

    const EXAMPLE_SOLUTION: &str = include_str!("../../data/day16_example.txt");

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

        assert_eq!(example_steps.len(), TIME_LIMIT);
        assert_eq!(example_steps[0].action, Action::Move(RoomId::new("DD")));
        assert_eq!(example_steps[0].pressure, 0);
        assert_eq!(example_steps[0].open_valves.len(), 0);

        let middle_step = &example_steps[17];
        assert_eq!(middle_step.action, Action::Move(RoomId::new("GG")));
        assert_eq!(middle_step.pressure, 76);
        assert_eq!(middle_step.open_valves.len(), 4);

        let last_step = &example_steps[29];
        assert_eq!(last_step.action, Action::Idle);
        assert_eq!(last_step.pressure, 81);
        assert_eq!(last_step.open_valves.len(), 6);

        let v = parse(SAMPLE);

        let mut total_pressure = 0;

        let mut open_valves = OpenValves::default();
        let mut player_location = RoomId::new("AA");

        for step in example_steps.iter() {
            println!("doing step {:?}", step);
            let current_flow = v.current_flow(&open_valves);
            total_pressure += current_flow;
            assert_eq!(step.pressure, current_flow);
            match &step.action {
                Action::Move(t) => {
                    player_location = *t;
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
        let v = parse(SAMPLE);

        let path = v.path_between_str("AA", "HH");
        assert_eq!(path.len(), 5);

        dbg!(&path);
    }

    #[test]
    fn test_permute_solve() {
        let v = parse(SAMPLE);
        let start_room = RoomId::new("AA");

        let rooms = v.rooms_with_valves();

        let one_path = to_path(&["DD", "BB", "JJ", "HH", "EE", "CC"]);
        let one_solution = solve(
            &v,
            &start_room,
            &to_ref_path(one_path.as_slice()),
            TIME_LIMIT,
        );
        assert_eq!(one_solution, 1651);

        let mut solutions: Vec<_> = rooms
            .iter()
            .permutations(rooms.len())
            .map(|path| {
                (
                    solve(&v, &start_room, path.as_slice(), TIME_LIMIT),
                    path.clone(),
                )
            })
            .collect();

        solutions.sort_by_key(|s| s.0);

        solutions.reverse();

        dbg!(&solutions[0..10]);

        assert_eq!(solutions[0].1, to_ref_path(one_path.as_slice()));
    }

    #[test]
    fn test_value_solve() {
        let v = parse(SAMPLE);
        let total_pressure = solver_solve(&v);

        assert_eq!(total_pressure, 1651);
    }
}
