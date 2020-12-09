use prelude::*;
use std::collections::VecDeque;

fn main() {
    do_main("inputs/day_09.txt");
}

fn do_main(filename: &str) {
    let input: Vec<i64> = read_lines_from_file(filename)
        .map(|line| line.parse().expect("not an integer"))
        .collect();

    let mut status = input.iter().take(25).collect::<VecDeque<_>>();

    let mut part1 = None;

    for i in input.iter().skip(25) {
        if !status
            .iter()
            .cartesian_product(status.iter())
            .any(|(&x, &y)| x + y == *i)
        {
            part1 = Some(i);
            break;
        }
        status.pop_front();
        status.push_back(i);
    }

    dbg!(part1);
}
