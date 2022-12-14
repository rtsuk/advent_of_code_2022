use nom::{
    branch::alt,
    character::complete::{char, u32},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::{Ordering, PartialOrd};

const DATA: &str = include_str!("../../data/day13.txt");

fn packet_value(input: &str) -> IResult<&str, Packet> {
    let (input, value) = u32(input)?;
    Ok((input, Packet::Value(value)))
}

fn bracketed(input: &str) -> IResult<&str, Packet> {
    let (input, values) = delimited(
        char('['),
        separated_list0(char(','), alt((packet_value, bracketed))),
        char(']'),
    )(input)?;
    Ok((input, Packet::List(values)))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Value(u32),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Value(v) => match other {
                Self::Value(ov) => v.cmp(ov),
                Self::List(ov) => {
                    for (v_val, o_val) in std::iter::once(Packet::Value(*v)).zip(ov.iter()) {
                        let ordering = v_val.cmp(o_val);
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                    1.cmp(&ov.len())
                }
            },
            Self::List(v) => match other {
                Self::Value(ov) => {
                    for (v_val, o_val) in v.iter().zip(std::iter::once(Packet::Value(*ov))) {
                        let ordering = v_val.cmp(&o_val);
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                    v.len().cmp(&1)
                }
                Self::List(ov) => {
                    for (v_val, o_val) in v.iter().zip(ov.iter()) {
                        let ordering = v_val.cmp(o_val);
                        if ordering != Ordering::Equal {
                            return ordering;
                        }
                    }
                    v.len().cmp(&ov.len())
                }
            },
        }
    }
}

impl From<&str> for Packet {
    fn from(s: &str) -> Self {
        println!("Packet::from {s}");
        if s.starts_with('[') {
            let contents = &s[1..s.len() - 1];
            let packets = contents.split(',').map(Packet::from).collect();
            Self::List(packets)
        } else {
            Self::Value(s.parse::<u32>().expect("usize"))
        }
    }
}

#[derive(Debug)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl From<&str> for PacketPair {
    fn from(s: &str) -> Self {
        let mut parts = s.lines();
        Self {
            left: bracketed(parts.next().expect("left")).expect("bracketed").1,
            right: bracketed(parts.next().expect("right"))
                .expect("bracketed")
                .1,
        }
    }
}

impl PacketPair {
    fn is_ordered(&self) -> bool {
        self.left.cmp(&self.right) == Ordering::Less
    }
}

fn parse(s: &str) -> Vec<PacketPair> {
    s.split("\n\n").map(PacketPair::from).collect()
}

fn calculate_marker_value(s: &str) -> usize {
    let packet_pairs = parse(s);
    let mut packets: Vec<_> = packet_pairs
        .into_iter()
        .flat_map(|pp| vec![pp.left, pp.right])
        .collect();

    let divider_1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    packets.push(divider_1.clone());
    let divider_2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);
    packets.push(divider_2.clone());
    packets.sort();
    let first_divider_pos = packets.iter().enumerate().find(|(_i, p)| **p == divider_1);
    let second_divider_pos = packets.iter().enumerate().find(|(_i, p)| **p == divider_2);

    (first_divider_pos.unwrap().0 + 1) * (second_divider_pos.unwrap().0 + 1)
}

fn main() {
    let packets = parse(DATA);
    let correct_indices: Vec<_> = packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| p.is_ordered().then_some(i + 1))
        .collect();
    println!("correct_indices = {correct_indices:?}");
    println!(
        "correct_indices sum = {}",
        correct_indices.iter().sum::<usize>()
    );

    let marker_values = calculate_marker_value(DATA);
    println!("marker_values = {marker_values}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_nom() {
        let r = bracketed("[3,2,1]");
        let r = r.unwrap();
        assert_eq!(
            r.1,
            Packet::List(vec![Packet::Value(3), Packet::Value(2), Packet::Value(1)])
        );

        let r = bracketed("[3,2,[1]]");
        let r = r.unwrap();
        assert_eq!(
            r.1,
            Packet::List(vec![
                Packet::Value(3),
                Packet::Value(2),
                Packet::List(vec![Packet::Value(1)])
            ])
        );
    }

    const SAMPLE: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

    #[test]
    fn test_parse() {
        let packet_pairs = parse(SAMPLE);
        assert_eq!(packet_pairs.len(), 8);
        assert_eq!(
            packet_pairs[0].left,
            Packet::List(vec![
                Packet::Value(1),
                Packet::Value(1),
                Packet::Value(3),
                Packet::Value(1),
                Packet::Value(1)
            ])
        );
        assert_eq!(
            packet_pairs[0].right,
            Packet::List(vec![
                Packet::Value(1),
                Packet::Value(1),
                Packet::Value(5),
                Packet::Value(1),
                Packet::Value(1)
            ])
        );
        assert_eq!(packet_pairs[5].left, Packet::List(vec![]));
        assert_eq!(packet_pairs[5].right, Packet::List(vec![Packet::Value(3),]));
    }

    #[test]
    fn test_part_1() {
        let packet_pairs = parse(SAMPLE);
        assert!(packet_pairs[0].is_ordered());
        assert!(packet_pairs[1].is_ordered());
        assert!(!packet_pairs[2].is_ordered());
        assert!(packet_pairs[3].is_ordered());
        assert!(!packet_pairs[4].is_ordered());
        assert!(packet_pairs[5].is_ordered());
        assert!(!packet_pairs[6].is_ordered());
        assert!(!packet_pairs[7].is_ordered());
    }

    #[test]
    fn test_part_2() {
        let marker_values = calculate_marker_value(SAMPLE);
        assert_eq!(marker_values, 140);
    }
}
