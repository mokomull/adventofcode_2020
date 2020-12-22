use prelude::*;

use Rule::*;

fn main() {
    do_main("inputs/day_19.txt");
}

fn do_main(filename: &str) {
    let (rules, messages) = parse_lines(&mut read_lines_from_file(filename));

    let rule0 = rules.get(&0).expect("rule 0 was not defined");
    let part1 = messages
        .iter()
        .filter(|&message| match_length(rule0, &rules, message).contains(&message.len()))
        .count();
    dbg!(part1);
    assert_eq!(part1, 129);

    let mut rules = rules;
    rules.insert(8, Alt(Seq(vec![42]).into(), Seq(vec![42, 8]).into()));
    rules.insert(
        11,
        Alt(Seq(vec![42, 31]).into(), Seq(vec![42, 11, 31]).into()),
    );
    let rule0 = rules.get(&0).unwrap();
    let part2 = messages
        .iter()
        .filter(|&message| match_length(rule0, &rules, message).contains(&message.len()))
        .count();
    dbg!(part2);
    assert_eq!(part2, 243);
}

#[derive(Debug)]
enum Rule {
    Char(u8),
    Seq(Vec<usize>),
    Alt(Box<Rule>, Box<Rule>),
}

fn parse_lines(
    mut input: &mut impl Iterator<Item = String>,
) -> (HashMap<usize, Rule>, Vec<Vec<u8>>) {
    let mut rules = HashMap::new();
    for line in &mut input {
        if line.is_empty() {
            break;
        }
        let mut halves = line.split(": ");
        let id = halves
            .next()
            .expect("missing id")
            .parse()
            .expect("not an integer");
        rules.insert(id, parse_rule(&halves.next().expect("missing rule body")));
    }

    let messages = input.map(|s| s.into_bytes()).collect();
    (rules, messages)
}

fn parse_rule(input: &str) -> Rule {
    if input.starts_with('"') {
        assert!(input.bytes().nth(2).unwrap() == b'"');
        return Char(input.bytes().nth(1).unwrap());
    }

    if input.contains('|') {
        let mut halves = input.splitn(2, "|");
        let left = parse_rule(halves.next().unwrap());
        let right = parse_rule(halves.next().unwrap());
        return Alt(left.into(), right.into());
    }

    Seq(input
        .split_whitespace()
        .map(|word| word.parse().expect("not an integer"))
        .collect())
}

fn match_length(rule: &Rule, rules: &HashMap<usize, Rule>, input: &[u8]) -> HashSet<usize> {
    match rule {
        Char(c) => {
            if input.get(0) == Some(c) {
                vec![1].drain(..).collect()
            } else {
                HashSet::new()
            }
        }
        Seq(sequence) => {
            let mut starting = HashSet::new();
            starting.insert(0);
            for i in sequence {
                starting = starting
                    .iter()
                    .flat_map(|so_far| {
                        match_length(
                            rules.get(i).expect("nonexistent rule"),
                            rules,
                            &input[*so_far..],
                        )
                        .into_iter()
                        .map(move |length| *so_far + length)
                    })
                    .collect();
            }
            starting
        }
        Alt(left, right) => {
            // with rule 8: 42 | 42 8, we need to get greedy and continue matching as many 42s as we
            // can.
            let l = match_length(left, rules, input);
            let r = match_length(right, rules, input);

            l.union(&r).cloned().collect()
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_19.txt");
    }
}
