use std::io::BufRead;

use itertools::Itertools;

fn main() {
    do_main("inputs/day_01.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut expenses = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Ok(expense) = line.as_str().trim().parse::<isize>() {
                expenses.push(expense);
            }
        }
    }

    let part1 = expenses
        .iter()
        .tuple_combinations()
        .filter(|&(x, y)| x + y == 2020)
        .map(|(x, y)| x * y)
        .next()
        .expect("there should have been a pair that added to 2020");
    println!("part 1: {}", part1);

    let part2 = expenses
        .iter()
        .tuple_combinations()
        .filter(|&(x, y, z)| x + y + z == 2020)
        .map(|(x, y, z)| x * y * z)
        .next()
        .expect("there should have been a triple that added to 2020");
    println!("part 2: {}", part2);

    assert_eq!(part1, 1019904);
    assert_eq!(part2, 176647680);
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_01.txt");
    }
}
