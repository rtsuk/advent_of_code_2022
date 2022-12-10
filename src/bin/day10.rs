use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    AddX(isize),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let mut parts = s.split(' ');
        let mnemonic = parts.next().expect("mnemonic");
        match mnemonic {
            "noop" => Instruction::Noop,
            "addx" => {
                let operand = parts.next().expect("operand");
                Instruction::AddX(operand.parse::<isize>().expect("operand as isize"))
            }
            _ => panic!("illegal mnemonic"),
        }
    }
}

type Program = Vec<Instruction>;

struct Cpu {
    program: Program,
    pub pc: usize,
    pub cycle: usize,
    remaining_cycles: usize,
    pub x: isize,
}

impl Cpu {
    pub fn new(program: Program) -> Self {
        let remaining_cycles = program[0].cycles();
        Self {
            program,
            pc: 0,
            cycle: 1,
            remaining_cycles,
            x: 1,
        }
    }

    pub fn running(&self) -> bool {
        self.pc < self.program.len()
    }

    pub fn clock(&mut self) {
        self.cycle += 1;
        self.remaining_cycles -= 1;
        if self.remaining_cycles == 0 {
            match self.program[self.pc] {
                Instruction::AddX(value) => {
                    self.x += value;
                }
                Instruction::Noop => (),
            }
            self.pc += 1;
            if self.running() {
                self.remaining_cycles = self.program[self.pc].cycles();
            }
        }
    }
}

const TARGET_CYCLES: &[usize] = &[20, 60, 100, 140, 180, 220];
const DATA: &str = include_str!("../../data/day10.txt");

fn parse(s: &str) -> Program {
    s.lines().map(Instruction::from).collect()
}

fn draw_screen(p: &Program) -> Vec<String> {
    let mut screen: Vec<String> = vec![];
    let mut cpu = Cpu::new(p.clone());
    while cpu.running() {
        let zero_based_cycle = cpu.cycle - 1;
        let column = (zero_based_cycle) % 40;
        let row = (zero_based_cycle) / 40;
        if row >= screen.len() {
            screen.push(String::new());
        }
        let sprite_range = cpu.x - 1..=cpu.x + 1;
        let pixel_display = if sprite_range.contains(&(column as isize)) {
            '#'
        } else {
            '.'
        };
        screen[row].push(pixel_display);
        cpu.clock();
    }
    screen
}

fn main() {
    let program = parse(DATA);

    let targets: HashSet<_> = TARGET_CYCLES.iter().collect();
    println!("targets  = {:?}", targets);

    let mut cpu = Cpu::new(program.clone());

    let mut signal_strength_sum = 0;
    while cpu.running() {
        if targets.contains(&cpu.cycle) {
            let signal_strength = cpu.x * cpu.cycle as isize;
            signal_strength_sum += signal_strength;
        }
        cpu.clock();
    }
    println!("signal_strength_sum = {}", signal_strength_sum);

    let screen = draw_screen(&program);
    println!("screen = {:#?}", screen);
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    const SAMPLE: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

    #[test]
    fn test_parse() {
        let program = parse(SAMPLE);
        assert_eq!(program.len(), 146);
    }

    #[test]
    fn test_part1() {
        const TARGET_X: &[usize] = &[21, 19, 18, 21, 16, 18];
        const TARGET_SIGNAL_STRENGTHS: &[usize] = &[420, 1140, 1800, 2940, 2880, 3960];
        let program = parse(SAMPLE);
        let mut cpu = Cpu::new(program);

        let targets: HashMap<usize, (usize, usize)> = TARGET_CYCLES
            .iter()
            .copied()
            .zip(
                TARGET_X
                    .iter()
                    .copied()
                    .zip(TARGET_SIGNAL_STRENGTHS.iter().copied()),
            )
            .collect();

        let mut signal_strength_sum = 0;
        while cpu.running() {
            if let Some(target) = targets.get(&cpu.cycle) {
                assert_eq!(target.0 as isize, cpu.x);
                let signal_strength = cpu.x * cpu.cycle as isize;
                assert_eq!(target.1 as isize, signal_strength);
                signal_strength_sum += signal_strength;
            }
            cpu.clock();
        }
        assert_eq!(signal_strength_sum, 13140);
    }

    #[test]
    fn test_part2() {
        let program = parse(SAMPLE);
        let expected = [
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ];
        let screen = draw_screen(&program);
        for (expected, line) in screen.iter().zip(expected.iter()) {
            assert_eq!(expected, line);
        }
    }
}
