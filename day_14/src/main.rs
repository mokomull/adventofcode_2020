use prelude::*;

use Instruction::*;

fn main() {
    do_main("inputs/day_14.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Instruction> = read_lines_from_file(filename)
        .map(|line| line.as_str().into())
        .collect();

    let mut memory = HashMap::new();
    let mut set = Vec::new();
    let mut clear = Vec::new();

    for i in &input {
        match i {
            Mask {
                set: new_set,
                clear: new_clear,
            } => {
                set = new_set.clone();
                clear = new_clear.clone();
            }
            Mem(address, value) => {
                let mut data = *value;

                for bit in &set {
                    data |= 1 << bit;
                }
                for bit in &clear {
                    data &= !(1 << bit);
                }

                memory.insert(*address, data);
            }
        }
    }

    let part1 = memory.values().sum::<u64>();
    dbg!(part1);
}

enum Instruction {
    Mask { set: Vec<u8>, clear: Vec<u8> },
    Mem(usize, u64),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Instruction {
        let mut words = input.split_whitespace();
        let op = words.next().expect("not enough words in line");
        let _equals = words.next().expect("no equals sign");
        let value = words.next().expect("no RHS in assignment");

        if op == "mask" {
            let mut set = Vec::new();
            let mut clear = Vec::new();

            for (i, action) in value.bytes().rev().enumerate() {
                match action {
                    b'1' => set.push(i as u8),
                    b'0' => clear.push(i as u8),
                    b'X' => (),
                    x => panic!("unknown byte in RHS: {}", x),
                }
            }
            return Mask { set, clear };
        } else if !op.starts_with("mem[") {
            panic!("I don't know what to do with {}", input);
        }

        let address = op
            .split(&['[', ']'][..])
            .nth(1)
            .expect("no address present")
            .parse()
            .expect("not an integer");
        Mem(address, value.parse().expect("invalid integer"))
    }
}
