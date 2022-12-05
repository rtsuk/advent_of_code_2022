const DATA: &str = include_str!("../../data/day5.txt");

struct Move {
    pub count: usize,
    pub source: usize,
    pub destination: usize,
}

struct StackMap {
    stacks: Vec<Vec<char>>,
}

fn main() {
    println!("top crates = {}", "");
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn test_parse() {
        let mut lines_iter = SAMPLE.lines();
        loop {
            if let Some(line) = lines_iter.next() {
                if line.len() == 0 {
                    break;
                }
                let chunks = line
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks(4)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>();

                let stacks: Vec<_> = chunks.iter().map(|s| s.chars().nth(1)).collect();
                dbg!(&stacks);
            }
        }
        todo!();
    }
}
