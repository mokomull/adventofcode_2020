use prelude::*;

use itertools::Itertools;

fn main() {
    do_main("inputs/day_07.txt");
}

fn do_main(filename: &str) {
    let input = read_lines_from_file(filename)
        .map(|line| parse_line(&line))
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
    assert_eq!(part1, 208);

    let mut bags_inside = HashMap::new();

    while !bags_inside.contains_key(&"shiny gold".to_owned()) {
        for (outer, inners) in &input {
            if bags_inside.contains_key(&outer) {
                continue;
            }

            let cost = inners
                .iter()
                .map(|(count, kind)| {
                    bags_inside
                        .get(kind)
                        .map(|&known_bag| count * (1usize + known_bag))
                })
                .fold(Some(0), |prev, cost_of_one_inner_bag| {
                    match (prev, cost_of_one_inner_bag) {
                        (Some(running_cost), Some(next_bag)) => Some(running_cost + next_bag),
                        _ => None,
                    }
                });

            if let Some(this_cost) = cost {
                bags_inside.insert(outer, this_cost);
            }
        }
    }

    let part2 = bags_inside.get(&"shiny gold".to_owned()).unwrap();
    dbg!(part2);
    assert_eq!(*part2, 1664);
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

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_07.txt");
    }
}
