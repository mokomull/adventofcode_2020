use std::collections::HashSet;
use std::io::BufRead;

use itertools::Itertools;

fn main() {
    do_main("inputs/day_07.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");

    let input = std::io::BufReader::new(file)
        .lines()
        .map(|line| parse_line(&line.expect("could not read a line")))
        .collect::<Vec<_>>();

    let mut can_contain_gold = HashSet::new();
    let mut to_search = std::iter::once("shiny gold").collect::<Vec<_>>();

    while !to_search.is_empty() {
        let target = to_search.pop().unwrap();

        for (outer_bag, _) in input.iter().filter(|&(_outer, inners)| {
            inners
                .iter()
                .find(|&(_count, kind)| kind == target)
                .is_some()
        }) {
            if can_contain_gold.insert(outer_bag) {
                to_search.push(outer_bag);
            }
        }
    }

    let part1 = can_contain_gold.len();
    dbg!(part1);
}

fn parse_line(line: &str) -> (String, Vec<(usize, String)>) {
    let mut halves = line.split(" bags contain ");
    let outer = halves
        .next()
        .expect("line did not name an outer bag")
        .to_owned();
    let inner = halves
        .next()
        .expect("line did not have any inner bags")
        .split_whitespace()
        .tuples()
        .map(|(number, adjective, color, _noun)| {
            (
                number.parse().expect("integer was malformed"),
                format!("{} {}", adjective, color),
            )
        })
        .collect();

    (outer, inner)
}
