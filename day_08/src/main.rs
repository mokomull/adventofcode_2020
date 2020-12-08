use std::collections::HashSet;
use std::io::{BufRead, BufReader};

// use itertools::Itertools;

fn main() {
    do_main("inputs/day_08.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let instructions: Vec<Instruction> = BufReader::new(file)
        .lines()
        .map(|line| line.expect("could not read a line").into())
        .collect();

    let mut acc = 0;
    let mut ip = 0;
    let mut ips = HashSet::new();

    let part1 = loop {
        if !ips.insert(ip) {
            break acc;
        }

        ip += 1;

        match instructions[ip] {
            Instruction::Acc(offset) => acc += offset,
            Instruction::Nop => {}
            Instruction::Jmp(offset) => {
                ip = ip.wrapping_add((offset - 1) as usize);
            }
        }
    };
    dbg!(part1);
}

enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

impl<T> From<T> for Instruction
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let value = value.as_ref();

        let mut halves = value.split_whitespace();
        let opcode = halves.next();
        let argument = halves.next();
        let argument = argument
            .expect("argument was not provided")
            .parse()
            .expect(&format!("invalid integer in line {}", value));
        match opcode {
            Some("acc") => Instruction::Acc(argument),
            Some("jmp") => Instruction::Jmp(argument),
            Some("nop") => Instruction::Nop,
            _ => panic!("could not parse {}", value),
        }
    }
}
