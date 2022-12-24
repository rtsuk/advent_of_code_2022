use anyhow::Error;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day20.txt");
const SAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;

#[derive(Debug, StructOpt)]
#[structopt(name = "day20", about = "Grove Positioning System")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

type Record = (usize, isize);
type List = Vec<Record>;

fn parse(s: &str, key: usize) -> Vec<Record> {
    s.lines()
        .map(|s| s.parse::<isize>().unwrap() * key as isize)
        .enumerate()
        .collect()
}

fn solve(mut data_list: List, count: usize) -> isize {
    let data_len = data_list.len() as isize;

    for _ in 0..count {
        for original_index in 0..data_list.len() {
            let index = data_list
                .iter()
                .position(|x| x.0 == original_index)
                .unwrap();
            let value = data_list[index].1;
            let new_index = index as isize + value;
            let new_index = new_index.rem_euclid(data_list.len() as isize - 1);
            let tmp = data_list.remove(index);
            data_list.insert(new_index as usize, tmp);
        }
    }

    let tests = [1000, 2000, 3000];

    let zero_position = data_list
        .iter()
        .copied()
        .position(|val| val.1 == 0)
        .expect("position");

    let mut sum = 0;
    for t in tests {
        let i = (zero_position + t) % (data_len as usize);
        let v = data_list[i];
        sum += v.1;
    }
    sum
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE }, 1);

    let sum = solve(file_contents, 1);

    println!("sum = {sum}");

    let file_contents = parse(if opt.puzzle_input { DATA } else { SAMPLE }, 811589153);
    let sum = solve(file_contents, 10);

    println!("sum = {sum}");

    // You guessed 8920 too high

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const _EXPECTED: &[[isize; 7]] = &[
        // Initial arrangement:
        [1, 2, -3, 3, -2, 0, 4],
        // 1 moves between 2 and -3:
        [2, 1, -3, 3, -2, 0, 4],
        // 2 moves between -3 and 3:
        [1, -3, 2, 3, -2, 0, 4],
        // -3 moves between -2 and 0:
        [1, 2, 3, -2, -3, 0, 4],
        // 3 moves between 0 and 4:
        [1, 2, -2, -3, 0, 3, 4],
        // -2 moves between 4 and 1:
        [1, 2, -3, 0, 3, 4, -2],
        // 0 does not move:
        [1, 2, -3, 0, 3, 4, -2],
        // 4 moves between -3 and 0:
        [1, 2, -3, 4, 0, 3, -2],
    ];

    #[test]
    fn test_parse() {
        let file_contents = parse(SAMPLE, 1);
        dbg!(&file_contents);
        assert_eq!(file_contents.len(), 7);
    }

    #[test]
    fn test_part_1() {
        let data = parse(SAMPLE, 1);
        let sum = solve(data, 1);
        assert_eq!(sum, 3);
    }

    #[test]
    fn test_part_2() {
        let data = parse(SAMPLE, 811589153);
        let sum = solve(data, 10);
        assert_eq!(sum, 1623178306);
    }
}
