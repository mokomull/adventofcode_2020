use prelude::*;

fn main() {
    do_main("inputs/day_10.txt");
}

fn do_main(filename: &str) {
    let mut input: Vec<i64> = read_lines_from_file(filename)
        .map(|s| s.parse().expect("not an integer"))
        .collect();

    // we always start with 0
    input.push(0);
    input.sort();
    let input = input;

    let mut one = 0;
    let mut three = 0;

    for (lower, upper) in input.iter().zip(input.iter().skip(1)) {
        match upper - lower {
            0 => panic!("zero: {}", lower),
            1 => one += 1,
            2 => panic!("two: {} - {}", lower, upper),
            3 => three += 1,
            x => panic!("way too many jump {}: {} to {}", x, lower, upper),
        }
    }

    let part1 = one * (three + 1); // + 1 due to the "built in" one being one-higher
    dbg!(part1);

    let mut ways = vec![0u64; input.len()];
    ways[0] = 1;
    for i in 1..input.len() {
        let this = input[i];
        for j in i.saturating_sub(3)..i {
            let prev = input[j];
            if prev < this - 3 {
                continue;
            }
            ways[i] += ways[j];
        }
    }
    dbg!(ways[ways.len() - 1]);
}
