#[derive(Default, Debug, Clone, Copy)]
struct Elf {
    pub index: usize,
    pub count: u32,
}

type ElfList = Vec<Elf>;

fn parse_input(value: &str) -> Vec<u32> {
    value
        .lines()
        .map(|s| s.parse::<u32>().unwrap_or_default())
        .collect()
}

fn make_elves(input_data: &str) -> ElfList {
    let values: Vec<_> = parse_input(input_data);

    let acc = vec![Vec::new()];
    let value_lists: Vec<Vec<u32>> = values.into_iter().fold(acc, |mut acc, x| {
        if x == 0 {
            acc.push(Vec::new());
        } else {
            acc.last_mut().unwrap().push(x);
        }
        acc
    });
    let mut counts: Vec<_> = value_lists
        .into_iter()
        .enumerate()
        .map(|(index, list)| Elf {
            index: index + 1,
            count: list.into_iter().sum::<u32>(),
        })
        .collect();
    counts.sort_by(|a, b| b.count.cmp(&a.count));
    counts
}

const PART1_DATA: &str = include_str!("../../data/day01.txt");

fn main() {
    let elves = make_elves(PART1_DATA);
    println!("best elf = {} cal {}", elves[0].index, elves[0].count);

    let top_3: u32 = elves[0..3].iter().map(|e| e.count).sum();
    println!("top 3 = {}", top_3);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

    #[test]
    fn test_parse() {
        dbg!(SAMPLE);
        let values: Vec<_> = parse_input(SAMPLE);
        assert_eq!(values.len(), 14);
        dbg!(&values);
        assert_eq!(values[0], 1000);
        assert_eq!(values[13], 10000);
    }

    #[test]
    fn test_sum() {
        let elves = make_elves(SAMPLE);
        assert_eq!(elves[0].index, 4);
    }
}
