#![allow(unused)]
use anyhow::Error;
use pathfinding::prelude::*;
use petgraph::{
    dot::{Config, Dot},
    graphmap::UnGraphMap,
};
use regex::Regex;
use std::collections::BTreeMap;
use structopt::StructOpt;

type Coord = i128;
type Point = (Coord, Coord);

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

fn letter_code_to_point(s: &str) -> Point {
    let values: Vec<_> = s.chars().map(|c| c as i128 - 'A' as i128).collect();
    (values[0], values[1])
}

#[derive(Debug)]
struct Room {
    name: String,
    location: Point,
    flow: usize,
    open: bool,
    tunnels: Vec<Point>,
}

impl Room {
    fn new(captures: regex::Captures) -> Self {
        let name = captures[1].to_string();
        let location = letter_code_to_point(&name);
        Self {
            name,
            location,
            flow: captures[2].parse::<usize>().expect("usize"),
            open: false,
            tunnels: captures[3].split(", ").map(letter_code_to_point).collect(),
        }
    }

    fn flow(&self) -> usize {
        if self.open {
            self.flow
        } else {
            0
        }
    }
}

type RoomSet = BTreeMap<String, Room>;

#[derive(Debug)]
enum Action {
    Move(String),
    Open(String),
    Idle,
}

#[derive(Default, Debug)]
struct Volcano {
    rooms: RoomSet,
    graph: UnGraphMap<Point, ()>,
    time: usize,
    player_room: String,
}

fn successors(point: &Point, graph: &UnGraphMap<Point, ()>) -> Vec<Point> {
    graph.neighbors(*point).collect()
}

impl Volcano {
    fn new(player_room: &str, rooms: RoomSet) -> Self {
        let graph = Self::make_graph(&rooms);
        Self {
            rooms,
            graph,
            time: 0,
            player_room: player_room.to_string(),
        }
    }

    fn make_graph(rooms: &RoomSet) -> UnGraphMap<Point, ()> {
        let edges: Vec<_> = rooms
            .values()
            .flat_map(|r| {
                r.tunnels
                    .iter()
                    .map(|t| (r.location, *t))
                    .collect::<Vec<_>>()
            })
            .collect();

        UnGraphMap::<Point, ()>::from_edges(&edges)
    }

    fn current_flow(&self) -> usize {
        self.rooms.values().map(Room::flow).sum()
    }

    fn path_between(&self, start: &str, end: &str) -> Vec<Point> {
        let start = letter_code_to_point(start);
        let end = letter_code_to_point(end);
        println!("from {start:?} to {end:?}");
        let graph = self.graph.clone();
        bfs(&start, |p| successors(p, &graph), |p| p == &end).unwrap_or_default()
    }
}

fn parse(s: &str) -> Volcano {
    let re = Regex::new(
        r"Valve ([A-Z][A-Z]) has flow rate=(\d+); tunnels* leads* to valves* ([A-Z, ]+)",
    )
    .expect("re");

    let rooms = re
        .captures_iter(s)
        .map(Room::new)
        .map(|r| (r.name.to_string(), r))
        .collect();

    Volcano::new("AA", rooms)
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

    let volcano = parse(if !opt.puzzle_input { SAMPLE } else { DATA });

    if opt.graph {
        println!(
            "{:?}",
            Dot::with_config(&volcano.graph, &[Config::EdgeNoLabel])
        );
    } else {
        todo!();
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
    fn test_volcano() {
        let mut v = parse(SAMPLE);

        let max_room = v
            .rooms
            .values()
            .filter(|r| !r.open)
            .max_by_key(|r| r.flow)
            .map(|r| r.name.to_string());
        dbg!(&max_room);

        assert_eq!(Some("HH"), max_room.as_ref().map(String::as_str));

        if let Some(max_room_name) = max_room.as_ref() {
            let r = v.rooms.get_mut(max_room_name).expect("room");
            r.open = true;
        }

        let max_room = v
            .rooms
            .values()
            .filter(|r| !r.open)
            .max_by_key(|r| r.flow)
            .map(|r| r.name.to_string());
        dbg!(&max_room);

        assert_eq!(Some("JJ"), max_room.as_ref().map(String::as_str));

        assert_eq!(v.current_flow(), 22);

        let path = v.path_between("AA", "HH");
        assert_eq!(path.len(), 6);
    }
}
