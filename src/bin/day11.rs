const DATA: &str = include_str!("../../data/day11.txt");

type WorryValue = u128;

fn monkey_label(s: Option<&str>) -> Option<usize> {
    s?.chars()
        .nth("Monkey ".len())?
        .to_digit(10)
        .map(|d| d as usize)
}

fn labeled_value(s: Option<&str>) -> Option<&str> {
    s?.split(':').last().map(str::trim)
}

fn comma_delimeted_list(s: Option<&str>) -> Option<Vec<WorryValue>> {
    Some(
        s?.split(',')
            .map(|s| s.trim().parse::<u128>().expect("u128"))
            .collect(),
    )
}

fn test_divisor(s: Option<&str>) -> Option<usize> {
    s?["divisible by ".len()..].parse::<usize>().ok()
}

fn target(s: Option<&str>) -> Option<usize> {
    s?["throw to monkey ".len()..].parse::<usize>().ok()
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Constant(WorryValue),
    Old,
}

impl Value {
    fn evaluate(&self, value: WorryValue) -> WorryValue {
        match self {
            Self::Constant(v) => *v,
            Self::Old => value,
        }
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        match s {
            "old" => Value::Old,
            _ => Value::Constant(s.trim().parse::<WorryValue>().expect("constant")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
}

impl Operation {
    fn evaluate(&self, a: WorryValue, b: WorryValue) -> WorryValue {
        match self {
            Self::Addition => a + b,
            Self::Multiplication => a * b,
        }
    }
}

impl From<&str> for Operation {
    //  Operation: new = old * old
    fn from(s: &str) -> Self {
        match s.trim() {
            "+" => Operation::Addition,
            "*" => Operation::Multiplication,
            _ => panic!("unknown operation"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Expression {
    lhs: Value,
    operation: Operation,
    rhs: Value,
}

impl Expression {
    fn apply(self, value: WorryValue) -> WorryValue {
        let left_value = self.lhs.evaluate(value);
        let right_value = self.rhs.evaluate(value);
        self.operation.evaluate(left_value, right_value)
    }
}

impl From<&str> for Expression {
    //  Operation: new = old * old
    fn from(s: &str) -> Self {
        let expression_parts: Vec<_> = s.split('=').map(str::trim).collect();
        let expression_value_parts: Vec<_> = expression_parts[1]
            .split(['+', '*'])
            .map(str::trim)
            .collect();
        let lhs = Value::from(expression_value_parts[0]);
        let operation = if s.contains('+') {
            Operation::Addition
        } else {
            Operation::Multiplication
        };
        let rhs = Value::from(expression_value_parts[1]);
        Self {
            lhs,
            operation,
            rhs,
        }
    }
}

#[derive(Debug)]
struct Throw {
    target: usize,
    item: WorryValue,
}

#[derive(Debug, Clone)]
struct Monkey {
    #[allow(unused)]
    index: usize,
    items: Vec<WorryValue>,
    expression: Expression,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
    inspection_count: u128,
}

impl Monkey {
    fn apply_expression(&mut self) {
        self.items
            .iter_mut()
            .for_each(|item| *item = self.expression.apply(*item));
    }

    fn decrease_worry(&mut self) {
        self.items.iter_mut().for_each(|item| *item /= 3);
    }

    fn modula(&mut self, value: WorryValue) {
        self.items.iter_mut().for_each(|item| *item %= value);
    }

    fn inspect_items(&mut self) -> Vec<Throw> {
        self.inspection_count += self.items.len() as u128;
        let test_divisor = self.test_divisor;
        let true_target = self.true_target;
        let false_target = self.false_target;
        let (for_true_target, for_false_target): (Vec<_>, Vec<_>) = self
            .items
            .iter()
            .partition(|item| *item % (test_divisor as WorryValue) == 0);
        let thrown_items: Vec<Throw> = for_true_target
            .iter()
            .map(|item| Throw {
                target: true_target,
                item: *item,
            })
            .chain(for_false_target.iter().map(|item| Throw {
                target: false_target,
                item: *item,
            }))
            .collect();

        self.items.clear();

        thrown_items
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let index = monkey_label(lines.next()).expect("monkey_label");
        let items = comma_delimeted_list(labeled_value(lines.next())).expect("items");
        let expression = Expression::from(labeled_value(lines.next()).expect("labeled_value"));
        let test_divisor = test_divisor(labeled_value(lines.next())).expect("test_divisor");
        let true_target = target(labeled_value(lines.next())).expect("true_target");
        let false_target = target(labeled_value(lines.next())).expect("false_target");
        Self {
            index,
            items,
            expression,
            test_divisor,
            true_target,
            false_target,
            inspection_count: 0,
        }
    }
}

type MonkeyList = Vec<Monkey>;

fn parse(s: &str) -> MonkeyList {
    s.split("\n\n").map(Monkey::from).collect()
}

fn execute_round_with_worry(monkeys: &mut MonkeyList, decrease_worry: bool) {
    let mut common_test = 1;

    if !decrease_worry {
        for monkey in monkeys.iter() {
            common_test *= monkey.test_divisor as WorryValue;
        }
    }

    for index in 0..monkeys.len() {
        monkeys[index].apply_expression();
        if decrease_worry {
            monkeys[index].decrease_worry();
        } else {
            // modula trick stolen from
            // https://github.com/samoylenkodmitry/AdventOfCode2022/blob/master/src/day11.rs
            // but I'm not sure why it works
            monkeys[index].modula(common_test);
        }
        let throws = monkeys[index].inspect_items();
        for throw in throws {
            monkeys[throw.target].items.push(throw.item);
        }
    }
}

fn execute_round(monkeys: &mut MonkeyList) {
    execute_round_with_worry(monkeys, true);
}

fn main() {
    let mut monkeys = parse(DATA);

    let mut second_monkeys = monkeys.clone();

    for _ in 0..20 {
        execute_round(&mut monkeys);
    }

    monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;
    println!("monkey_business = {}", monkey_business);

    for round in 0..10_000 {
        if round % 100 == 0 {
            println!("round {}", round)
        }
        execute_round_with_worry(&mut second_monkeys, false);
    }

    second_monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

    let monkey_business = second_monkeys[0].inspection_count * second_monkeys[1].inspection_count;
    println!("monkey_business part2 = {}", monkey_business);
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    fn compare_worries(worries: &Vec<WorryValue>, expected: &[usize]) {
        assert_eq!(worries.len(), expected.len());
        for i in 0..worries.len() {
            assert_eq!(worries[i] as usize, expected[i]);
        }
    }

    #[test]
    fn test_parse() {
        let monkeys = parse(SAMPLE);
        assert_eq!(monkeys.len(), 4);
        compare_worries(&monkeys[0].items, &[79, 98]);
        assert_eq!(monkeys[0].test_divisor, 23);
        assert_eq!(monkeys[0].true_target, 2);
        assert_eq!(monkeys[0].false_target, 3);
    }

    #[test]
    fn test_part1() {
        let mut monkeys = parse(SAMPLE);
        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[20, 23, 27, 26]);
        compare_worries(&monkeys[1].items, &[2080, 25, 167, 207, 401, 1046]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[695, 10, 71, 135, 350]);
        compare_worries(&monkeys[1].items, &[43, 49, 58, 55, 362]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[16, 18, 21, 20, 122]);
        compare_worries(&monkeys[1].items, &[1468, 22, 150, 286, 739]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[491, 9, 52, 97, 248, 34]);
        compare_worries(&monkeys[1].items, &[39, 45, 43, 258]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[15, 17, 16, 88, 1037]);
        compare_worries(&monkeys[1].items, &[20, 110, 205, 524, 72]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[8, 70, 176, 26, 34]);
        compare_worries(&monkeys[1].items, &[481, 32, 36, 186, 2190]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[162, 12, 14, 64, 732, 17]);
        compare_worries(&monkeys[1].items, &[148, 372, 55, 72]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[51, 126, 20, 26, 136]);
        compare_worries(&monkeys[1].items, &[343, 26, 30, 1546, 36]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[116, 10, 12, 517, 14]);
        compare_worries(&monkeys[1].items, &[108, 267, 43, 55, 288]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[91, 16, 20, 98]);
        compare_worries(&monkeys[1].items, &[481, 245, 22, 26, 1092, 30]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[83, 44, 8, 184, 9, 20, 26, 102]);
        compare_worries(&monkeys[1].items, &[110, 36]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        execute_round(&mut monkeys);
        compare_worries(&monkeys[0].items, &[10, 12, 14, 26, 34]);
        compare_worries(&monkeys[1].items, &[245, 93, 53, 199, 115]);
        compare_worries(&monkeys[2].items, &[]);
        compare_worries(&monkeys[3].items, &[]);

        assert_eq!(monkeys[0].inspection_count, 101);
        assert_eq!(monkeys[1].inspection_count, 95);
        assert_eq!(monkeys[2].inspection_count, 7);
        assert_eq!(monkeys[3].inspection_count, 105);

        monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

        let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;
        assert_eq!(monkey_business, 10605);
    }

    #[test]
    fn test_part2() {
        let mut monkeys = parse(SAMPLE);
        execute_round_with_worry(&mut monkeys, false);

        assert_eq!(monkeys[0].inspection_count, 2);
        assert_eq!(monkeys[1].inspection_count, 4);
        assert_eq!(monkeys[2].inspection_count, 3);
        assert_eq!(monkeys[3].inspection_count, 6);

        for _ in 1..20 {
            execute_round_with_worry(&mut monkeys, false);
        }

        assert_eq!(monkeys[0].inspection_count, 99);
        assert_eq!(monkeys[1].inspection_count, 97);
        assert_eq!(monkeys[2].inspection_count, 8);
        assert_eq!(monkeys[3].inspection_count, 103);

        for _ in 20..10_000 {
            execute_round_with_worry(&mut monkeys, false);
        }

        monkeys.sort_by(|a, b| b.inspection_count.cmp(&a.inspection_count));

        let monkey_business = monkeys[0].inspection_count * monkeys[1].inspection_count;
        assert_eq!(monkey_business, 2713310158);
    }
}
