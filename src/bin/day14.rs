const DATA: &str = include_str!("../../data/day14.txt");

fn parse(s: &str)  {
}

fn main() {
    let _ = parse(DATA);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_parse() {
	}
}
