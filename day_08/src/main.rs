use prelude::*;

fn main() {
    do_main("inputs/day_08.txt");
}

fn do_main(filename: &str) {
    let instructions: Vec<Instruction> = read_lines_from_file(filename).map(Into::into).collect();

    let part1 = run(&instructions);
    dbg!(&part1);
    assert_eq!(part1, Termination::InfiniteLoop(1675));

    let mut instructions = instructions;

    let mut part2 = None;

    for i in 0..instructions.len() {
        let new = match instructions[i] {
            Instruction::Nop(offset) => Instruction::Jmp(offset),
            Instruction::Jmp(offset) => Instruction::Nop(offset),
            _ => continue,
        };
        let old = std::mem::replace(&mut instructions[i], new);

        if let Termination::EndOfProgram(acc) = run(&instructions) {
            part2 = Some(acc);
        }

        instructions[i] = old;
    }

    dbg!(part2);
    assert_eq!(part2, Some(1532));
}

#[derive(Debug, PartialEq)]
enum Termination {
    InfiniteLoop(isize),
    EndOfProgram(isize),
}

fn run(instructions: &[Instruction]) -> Termination {
    let mut acc = 0;
    let mut ip = 0;
    let mut ips = HashSet::new();

    loop {
        if !ips.insert(ip) {
            return Termination::InfiniteLoop(acc);
        }
        if ip >= instructions.len() {
            return Termination::EndOfProgram(acc);
        }

        match instructions[ip] {
            Instruction::Acc(offset) => acc += offset,
            Instruction::Nop(_) => {}
            Instruction::Jmp(offset) => {
                ip = ip.wrapping_add((offset - 1) as usize);
            }
        }

        ip += 1;
    }
}

enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
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
            .unwrap_or_else(|_| panic!("invalid integer in line {}", value));
        match opcode {
            Some("acc") => Instruction::Acc(argument),
            Some("jmp") => Instruction::Jmp(argument),
            Some("nop") => Instruction::Nop(argument),
            _ => panic!("could not parse {}", value),
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_08.txt");
    }
}
