use anyhow::Error;
use structopt::StructOpt;

const DATA: &str = include_str!("../../data/day25.txt");
const SAMPLE: &str = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

fn snafu_digit(c: char) -> isize {
    match c {
        '1' => 1,
        '2' => 2,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("illegal snafu digit"),
    }
}

fn to_snafu_digit(i: isize) -> char {
    match i {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '=',
        4 => '-',
        _ => panic!("illegal snafu digit"),
    }
}

fn parse_snafu(s: &str) -> isize {
    if s.is_empty() {
        return 0;
    }
    let max_value = 5isize.pow(s.len() as u32);

    let value: isize = s
        .chars()
        .map(snafu_digit)
        .scan(max_value, |max_value, digit_value| {
            *max_value /= 5;
            Some(digit_value * *max_value)
        })
        .sum();
    value
}

fn to_snafu_string(v: isize) -> String {
    let snafu_digits: Vec<char> = std::iter::repeat(())
        .scan(v, |current_value, _| {
            let mut v = *current_value;
            if v > 0 {
                let amount_to_encode = v % 5;
                let digit = to_snafu_digit(amount_to_encode);
                if amount_to_encode >= 3 {
                    v += 5
                }
                v /= 5;
                *current_value = v;
                Some(digit)
            } else {
                None
            }
        })
        .collect();
    snafu_digits.iter().rev().collect::<String>()
}

fn parse(s: &str) -> Vec<String> {
    s.lines().map(str::to_string).collect()
}

fn solve_part_1(s: &[String]) -> String {
    let values: Vec<isize> = s.iter().map(String::as_str).map(parse_snafu).collect();
    let sum: isize = values.iter().sum();
    to_snafu_string(sum)
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day25", about = "Full of Hot Air")]
struct Opt {
    /// Use puzzle input instead of the sample
    #[structopt(short, long)]
    puzzle_input: bool,
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();

    let value_list = parse(if opt.puzzle_input { DATA } else { SAMPLE });

    let p1 = solve_part_1(&value_list);
    println!("part 1  = {p1}");

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::assert_equal;

    const EXPECTED: &[isize] = &[1747, 906, 198, 11, 201, 31, 1257, 32, 353, 107, 7, 3, 37];

    #[test]
    fn test_parse() {
        let value_list = parse(SAMPLE);
        let values: Vec<isize> = value_list
            .iter()
            .map(String::as_str)
            .map(parse_snafu)
            .collect();
        assert_equal(values.iter(), EXPECTED.iter());

        let sum: isize = values.iter().sum();
        assert_eq!(sum, 4890);

        assert_eq!(to_snafu_string(sum).as_str(), "2=-1=0");
    }
}
