use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::iter::zip;
use std::collections::{HashSet, VecDeque};
use regex::Regex;

#[derive(Clone, Debug)]
struct Program {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
    instructions: Vec<i8>,
    output: Vec<i64>,
}

impl Default for Program {
    fn default() -> Program {
        Program {
            a: 0,
            b: 0,
            c: 0,
            pc: 0,
            instructions: Vec::new(),
            output: Vec::new(),
        }
    }
}

impl Program {
    fn new(contents: String) -> Program {
        let int_regex = Regex::new(r"-?\d+").unwrap();
        let mut group_split = contents.split("\n\n");
        let mut register_split = int_regex.find_iter(group_split.next().unwrap());

        Program {
            a: register_split.next().unwrap().as_str().parse::<i64>().unwrap(),
            b: register_split.next().unwrap().as_str().parse::<i64>().unwrap(),
            c: register_split.next().unwrap().as_str().parse::<i64>().unwrap(),
            instructions: Vec::from_iter(int_regex.find_iter(group_split.next().unwrap()).map(|m| m.as_str().parse::<i8>().unwrap())),
            ..Default::default()
        }
    }

    fn combo_operand(&self, op: &i8) -> i64 {
        match op {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand: {}", op)
        }
    }

    fn run(&mut self) {
        loop {
            let mut this_instruction = self.instructions.windows(2).skip(self.pc);
            if let Some([opcode, operand]) = this_instruction.next() {
                self.pc = match opcode {
                    0 => {self.a >>= self.combo_operand(operand); self.pc + 2},
                    1 => {self.b ^= *operand as i64; self.pc + 2},
                    2 => {self.b = self.combo_operand(operand) & 0x7; self.pc + 2},
                    3 => {
                        if self.a != 0 {
                            *operand as usize
                        } else {
                            self.pc + 2
                        }
                    },
                    4 => {self.b ^= self.c; self.pc + 2},
                    5 => {self.output.push(self.combo_operand(operand) & 0x7); self.pc + 2},
                    6 => {self.b = self.a >> self.combo_operand(operand); self.pc + 2},
                    7 => {self.c = self.a >> self.combo_operand(operand); self.pc + 2},
                    _ => panic!("Unknown opcode: {}", opcode)
                };
            } else {
                break;
            }
        }
    }
}

fn part1(contents: String) -> String {
    let mut prog = Program::new(contents);
    prog.run();
    return prog.output.iter().map(|out| out.to_string()).reduce(|s, out| format!("{},{}", s, out)).unwrap();
}

fn part2(contents: String) -> Option<i64> {
    let orig_prog = Program::new(contents);
    let mut checking: VecDeque<i64> = VecDeque::from_iter(0..(1<<3));
    let mut checked: HashSet<i64> = HashSet::new();

    while let Some(a) = checking.pop_front() {
        let mut prog = orig_prog.clone();
        prog.a = a;
        prog.run();
        
        checked.insert(a);
        if zip(prog.output.iter().rev(), orig_prog.instructions.iter().rev().map(|v| *v as i64)).all(|(a, b)| *a == b) {
            if prog.output.len() == orig_prog.instructions.len() {
                return Some(a);
            } else {
                for n in (0..(1<<3)).map(|n| (a << 3) + n).filter(|n| !checked.contains(n)) {
                    checking.push_back(n);
                }
            }
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents).unwrap(), 117440);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2024".to_string();
    let day = "17".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nProgram output: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLowest Register A value to output itself: {}\nRan in {:.5?}",
        part2(contents.clone()).unwrap(),
        part2_timer.elapsed()
    );
}