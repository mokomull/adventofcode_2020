use prelude::*;

use Instruction::*;

fn main() {
    do_main("inputs/day_12.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Instruction> = read_lines_from_file(filename)
        .map(|s| s.as_str().into())
        .collect();

    // Using the Cartesian plane as a coordinate system: East is +x, North is +y.
    let mut x = 0;
    let mut y = 0;

    // The ship starts facing east.
    let mut dx = 1;
    let mut dy = 0;

    for i in &input {
        match i {
            North(d) => y += *d as i32,
            South(d) => y -= *d as i32,
            East(d) => x += *d as i32,
            West(d) => x -= *d as i32,
            Forward(d) => {
                x += *d as i32 * dx;
                y += *d as i32 * dy
            }
            Left(d) => {
                // this problem gets harder if we get off the axes
                assert!(*d % 90 == 0);

                for _ in 0..(*d / 90) {
                    // [1 0] -> [0 -1] so the transformation is [0 -1] * [dx]
                    // [0 1]    [1  0]                          [1  0]   [dy]
                    let newdx = -dy;
                    let newdy = dx;
                    dx = newdx;
                    dy = newdy;
                }
            }
            Right(d) => {
                assert!(*d % 90 == 0);
                for _ in 0..(*d / 90) {
                    // same thing except [1 0] -> [0  1]
                    //                   [0 1]    [-1 0]
                    let newdx = dy;
                    let newdy = -dx;
                    dx = newdx;
                    dy = newdy;
                }
            }
        }
    }

    let part1 = x.abs() + y.abs();
    dbg!(part1);
}

enum Instruction {
    North(u32),
    South(u32),
    East(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Instruction {
        let value = input[1..].parse().expect("input was not an integer");

        match input.chars().nth(0).expect("input was empty") {
            'N' => North(value),
            'S' => South(value),
            'E' => East(value),
            'W' => West(value),
            'L' => Left(value),
            'R' => Right(value),
            'F' => Forward(value),
            x => panic!("Unexpected instruction: {}", x),
        }
    }
}
