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
    assert_eq!(part1, Some(&22477624));

    let mut part2 = None;

    let mut sums = input.clone();

    'out: for distance in 0..sums.len() {
        for (i, sum) in sums.iter_mut().enumerate() {
            if sum == part1.unwrap() && distance > 0 {
                let range = input.iter().skip(i).take(distance + 1);
                let min = range.clone().min().expect("range was empty");
                let max = range.max().expect("range was empty");
                part2 = Some(min + max);
                break 'out;
            }

            if let Some(new) = input.get(i + distance + 1) {
                *sum = *sum + new;
            }
        }

        // the last one will always no longer have a pairing
        sums.pop();
    }

    dbg!(part2);
    assert_eq!(part2, Some(2980044));
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_09.txt");
    }
}
