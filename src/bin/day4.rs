use std::ops::Range;

type Asssignment = Range<usize>;

#[derive(Debug)]
struct Elf {
    assignment: Asssignment,
}

impl Elf {
    pub fn contains(&self, other: &Elf) -> bool {
        self.assignment.start <= other.assignment.start
            && self.assignment.end >= other.assignment.end
    }

    pub fn overlaps(&self, other: &Elf) -> bool {
        if self.assignment.contains(&other.assignment.start) {
            return true;
        }

        let other_last = other.assignment.end - 1;
        self.assignment.contains(&other_last)
    }
}

impl From<&str> for Elf {
    fn from(s: &str) -> Self {
        let mut range_limits = s.split('-').map(str::parse).map(Result::unwrap_or_default);
        let start = range_limits.next().expect("start");
        let inclusive_end = range_limits.next().expect("inclusive_end");
        Self {
            assignment: start..inclusive_end + 1,
        }
    }
}

#[derive(Debug)]
struct ElfPair {
    first: Elf,
    second: Elf,
}

impl ElfPair {
    pub fn fully_contained(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    pub fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

impl From<&str> for ElfPair {
    fn from(s: &str) -> Self {
        let mut elfs = s.split(',').map(Elf::from);
        Self {
            first: elfs.next().expect("first"),
            second: elfs.next().expect("first"),
        }
    }
}

fn parse_pairs(s: &str) -> Vec<ElfPair> {
    s.lines().map(ElfPair::from).collect()
}

fn count_fully_contained_pairs(pairs: &[ElfPair]) -> usize {
    pairs
        .iter()
        .map(ElfPair::fully_contained)
        .map(usize::from)
        .sum()
}

fn count_overlapping_pairs(pairs: &[ElfPair]) -> usize {
    pairs
        .iter()
        .map(ElfPair::overlaps)
        .map(usize::from)
        .sum()
}

const DATA: &str = include_str!("../../data/day4.txt");

fn main() {
    let pairs = parse_pairs(DATA);
    let fully = count_fully_contained_pairs(&pairs);
    println!("assignment pairs = {}", fully);
    let overlap = count_overlapping_pairs(&pairs);
    println!("overlap pairs = {}", overlap);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_parse() {
        let pairs = parse_pairs(SAMPLE);
        assert_eq!(pairs.len(), 6);
        let first_pair = &pairs[0];
        assert_eq!(first_pair.first.assignment, 2..5);
        assert_eq!(first_pair.second.assignment, 6..9);

        let pen_pair = &pairs[4];
        assert_eq!(pen_pair.first.assignment, 6..7);
        assert_eq!(pen_pair.second.assignment, 4..7);

        let last_pair = &pairs[5];
        assert_eq!(last_pair.first.assignment, 2..7);
        assert_eq!(last_pair.second.assignment, 4..9);
    }

    #[test]
    fn test_count_fully_contained_pairs() {
        let pairs = parse_pairs(SAMPLE);
        let fully = count_fully_contained_pairs(&pairs);
        assert_eq!(fully, 2);
    }

    #[test]
    fn test_overlapping_pairs() {
        let pairs = parse_pairs(SAMPLE);
        let fully = count_overlapping_pairs(&pairs);
        assert_eq!(fully, 4);
    }
}
