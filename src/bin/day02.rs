const PART1_DATA: &str = include_str!("../../data/day02.txt");

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Play {
    #[default]
    Rock,
    Paper,
    Scissors,
}

impl Play {
    pub fn win(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn lose(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    pub fn draw(&self) -> Self {
        *self
    }

    pub fn shape_score(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn outcome_score(&self, other: Play) -> usize {
        match self {
            Self::Rock => match other {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6,
            },
            Self::Paper => match other {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0,
            },
            Self::Scissors => match other {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3,
            },
        }
    }
}

impl From<&str> for Play {
    fn from(input: &str) -> Self {
        match input {
            "A" | "X" => Play::Rock,
            "B" | "Y" => Play::Paper,
            "C" | "Z" => Play::Scissors,
            _ => Play::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum DesiredOutcome {
    #[default]
    Lose,
    Draw,
    Win,
}

impl From<&str> for DesiredOutcome {
    fn from(input: &str) -> Self {
        match input {
            "X" => DesiredOutcome::Lose,
            "Y" => DesiredOutcome::Draw,
            "Z" => DesiredOutcome::Win,
            _ => DesiredOutcome::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Turn {
    them: Play,
    me: Play,
}

impl Turn {
    pub fn score(&self) -> usize {
        self.me.shape_score() + self.me.outcome_score(self.them)
    }
}

impl From<&str> for Turn {
    fn from(input: &str) -> Self {
        let mut parts = input.split(' ');
        Turn {
            them: parts.next().map(Play::from).unwrap_or_default(),
            me: parts.next().map(Play::from).unwrap_or_default(),
        }
    }
}

impl From<&TurnWithOutcome> for Turn {
    fn from(input: &TurnWithOutcome) -> Self {
        let them = input.them;
        let me = match input.me {
            DesiredOutcome::Win => them.lose(),
            DesiredOutcome::Lose => them.win(),
            DesiredOutcome::Draw => them.draw(),
        };
        Self { me, them }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TurnWithOutcome {
    them: Play,
    me: DesiredOutcome,
}

impl From<&str> for TurnWithOutcome {
    fn from(input: &str) -> Self {
        let mut parts = input.split(' ');
        TurnWithOutcome {
            them: parts.next().map(Play::from).unwrap_or_default(),
            me: parts.next().map(DesiredOutcome::from).unwrap_or_default(),
        }
    }
}

fn parse_input(value: &str) -> Vec<Turn> {
    value.lines().map(Turn::from).collect()
}

fn parse_input_2(value: &str) -> Vec<TurnWithOutcome> {
    value.lines().map(TurnWithOutcome::from).collect()
}

fn make_turns(turns: Vec<TurnWithOutcome>) -> Vec<Turn> {
    turns.iter().map(Turn::from).collect()
}

fn calculate_score(turns: Vec<Turn>) -> usize {
    turns.iter().map(Turn::score).sum()
}

fn main() {
    let turns: Vec<_> = parse_input(PART1_DATA);
    let score = calculate_score(turns);
    println!("score = {}", score);

    let turns: Vec<_> = parse_input_2(PART1_DATA);
    let turns = make_turns(turns);
    let score = calculate_score(turns);
    println!("score = {}", score);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"A Y
B X
C Z
"#;

    #[test]
    fn test_parse() {
        let turns: Vec<_> = parse_input(SAMPLE);
        assert_eq!(turns.len(), 3);
        dbg!(&turns);
        assert_eq!(turns[0].me, Play::Paper);
    }

    #[test]
    fn test_score() {
        let turns: Vec<_> = parse_input(SAMPLE);
        let score = calculate_score(turns);
        assert_eq!(score, 15);
    }

    #[test]
    fn test_score_part2() {
        let turns: Vec<_> = parse_input_2(SAMPLE);
        let turns = make_turns(turns);
        let score = calculate_score(turns);
        assert_eq!(score, 12);
    }
}
