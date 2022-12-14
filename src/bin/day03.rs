use std::collections::HashSet;

const DATA: &str = include_str!("../../data/day03.txt");

fn as_priority(c: char, base_char: char, base_value: usize) -> usize {
    c as usize - base_char as usize + base_value
}

#[derive(Debug)]
struct Item(pub char);

impl Item {
    pub fn priority(&self) -> usize {
        match self.0 {
            'A'..='Z' => as_priority(self.0, 'A', 27),
            'a'..='z' => as_priority(self.0, 'a', 1),
            _ => panic!("unsupported item priority"),
        }
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Self(c)
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: [String; 2],
}

impl Rucksack {
    pub fn misplaced_type(&self) -> Item {
        let contents_0: HashSet<_> = self.compartments[0].chars().collect();
        let contents_1: HashSet<_> = self.compartments[1].chars().collect();
        let mut misplaced = contents_0.intersection(&contents_1);
        Item::from(misplaced.next().copied().expect("misplaced"))
    }

    pub fn all_types(&self) -> HashSet<char> {
        let contents_0: HashSet<_> = self.compartments[0].chars().collect();
        let contents_1: HashSet<_> = self.compartments[1].chars().collect();
        contents_0.union(&contents_1).copied().collect()
    }
}

impl From<&str> for Rucksack {
    fn from(s: &str) -> Self {
        let len = s.len();
        assert!(len % 2 == 0);
        let slice = len / 2;
        Self {
            compartments: [s[0..slice].to_string(), s[slice..].to_string()],
        }
    }
}

fn parse_rucksacks(s: &str) -> Vec<Rucksack> {
    s.lines().map(Rucksack::from).collect()
}

fn sum_rucksacks(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(Rucksack::misplaced_type)
        .map(|item| item.priority())
        .sum()
}

fn find_badge(rucksacks: &[Rucksack]) -> char {
    let mut intersection: Option<HashSet<char>> = None;
    for sack in rucksacks {
        let all_types = sack.all_types();
        if let Some(inter) = intersection {
            intersection = Some(inter.intersection(&all_types).copied().collect())
        } else {
            intersection = Some(all_types);
        }
    }

    let intersection = intersection.expect("intersection");
    assert_eq!(intersection.len(), 1);
    intersection.iter().next().copied().unwrap()
}

fn main() {
    let rucksacks = parse_rucksacks(DATA);
    let sum = sum_rucksacks(&rucksacks);
    println!("sum of the priorities = {sum}",);

    let mut priority = 0;
    for set in rucksacks.chunks(3) {
        let badge = find_badge(set);
        let badge_item = Item(badge);
        priority += badge_item.priority();
    }

    println!("sum of badge priorities = {priority}");
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_priority() {
        assert_eq!(Item::from('A').priority(), 27);
        assert_eq!(Item::from('D').priority(), 30);
        assert_eq!(Item::from('a').priority(), 1);
        assert_eq!(Item::from('b').priority(), 2);
    }

    #[test]
    fn test_parse() {
        let rucksacks = parse_rucksacks(SAMPLE);
        assert_eq!(rucksacks.len(), 6);
        let sack_1 = &rucksacks[0];
        assert_eq!(sack_1.compartments[0], "vJrwpWtwJgWr");
        assert_eq!(sack_1.compartments[1], "hcsFMMfFFhFp");
    }

    #[test]
    fn test_misplaced() {
        let sack = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let misplaced = sack.misplaced_type();
        assert_eq!(misplaced.0, 'p');

        let sack = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let misplaced = sack.misplaced_type();
        assert_eq!(misplaced.0, 'L');
    }

    #[test]
    fn test_sum_of_misplaced() {
        let rucksacks = parse_rucksacks(SAMPLE);
        let sum = sum_rucksacks(&rucksacks);
        assert_eq!(sum, 157);
    }

    #[test]
    fn test_find_group() {
        const BADGES: &[char] = &['r', 'Z'];
        let rucksacks = parse_rucksacks(SAMPLE);
        for (index, set) in rucksacks.chunks(3).enumerate() {
            let badge = find_badge(set);
            assert_eq!(badge, BADGES[index]);
        }
    }
}
