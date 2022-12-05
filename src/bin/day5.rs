const DATA: &str = include_str!("../../data/day5.txt");

#[derive(Debug, Default)]
struct Move {
    pub count: usize,
    pub source: usize,
    pub destination: usize,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let parts: Vec<_> = s.split(' ').collect();
        assert_eq!(parts.len(), 6);
        assert_eq!(parts[0], "move");
        let count = parts[1].parse::<usize>().expect("count");
        assert!(count > 0);
        assert_eq!(parts[2], "from");
        let source = parts[3].parse::<usize>().expect("source");
        assert!(source > 0);
        assert_eq!(parts[4], "to");
        let destination = parts[5].parse::<usize>().expect("destination");
        assert!(destination > 0);
        Self {
            count,
            source: source - 1,
            destination: destination - 1,
        }
    }
}

#[derive(Debug, Default, Clone)]
struct StackMap {
    stacks: Vec<Vec<char>>,
}

impl StackMap {
    pub fn add_item(&mut self, index: usize, item: char) {
        if self.stacks.len() <= index {
            self.stacks.resize_with(index + 1, Default::default);
        }
        let stack = &mut self.stacks[index];
        stack.push(item);
    }

    pub fn execute(&mut self, move_order: &Move) {
        for _ in 0..move_order.count {
            let source_range = 0..1;
            let source: Vec<_> = self.stacks[move_order.source]
                .splice(source_range, [])
                .collect();
            self.stacks[move_order.destination].splice(0..0, source);
        }
    }

    pub fn execute_in_order(&mut self, move_order: &Move) {
        let source_range = 0..move_order.count;
        let source: Vec<_> = self.stacks[move_order.source]
            .splice(source_range, [])
            .collect();
        self.stacks[move_order.destination].splice(0..0, source);
    }

    pub fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.first())
            .collect()
    }
}

fn parse_data(data: &str) -> (StackMap, Vec<Move>) {
    let mut lines_iter = data.lines();
    let mut stack_map = StackMap::default();
    loop {
        if let Some(line) = lines_iter.next() {
            if line.is_empty() {
                break;
            }
            let chunks = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            let stacks: Vec<_> = chunks.iter().map(|s| s.chars().nth(1)).collect();
            for (index, item) in stacks.iter().enumerate() {
                if let Some(item) = item {
                    if item.is_ascii_alphabetic() {
                        stack_map.add_item(index, *item);
                    }
                }
            }
        }
    }

    let moves = lines_iter.map(Move::from).collect();

    (stack_map, moves)
}

fn main() {
    let (mut map, moves) = parse_data(DATA);

    let mut map_in_order = map.clone();

    for move_order in &moves {
        map.execute(move_order);
    }
    println!("top crates = {}", map.top_crates());

    for move_order in &moves {
        map_in_order.execute_in_order(move_order);
    }
    println!("top crates 9001 = {}", map_in_order.top_crates());
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
        let (map, moves) = parse_data(SAMPLE);
        assert_eq!(map.stacks.len(), 3);
        assert_eq!(map.stacks[0], ['N', 'Z']);
        assert_eq!(map.stacks[1], ['D', 'C', 'M']);
        assert_eq!(map.stacks[2], ['P']);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    fn test_move_commands() {
        let (mut map, moves) = parse_data(SAMPLE);
        for move_order in &moves {
            map.execute(move_order);
        }
        assert_eq!(&map.top_crates(), "CMZ");
    }

    #[test]
    fn test_move_in_order_commands() {
        let (mut map, moves) = parse_data(SAMPLE);
        for move_order in &moves {
            map.execute_in_order(move_order);
        }
        assert_eq!(&map.top_crates(), "MCD");
    }
}
