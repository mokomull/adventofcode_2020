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
        .filter(|&message| match_length(rule0, &rules, message) == Some(message.len()))
        .count();
    dbg!(part1);

    let mut rules = rules;
    rules.insert(8, Alt(Seq(vec![42]).into(), Seq(vec![42, 8]).into()));
    rules.insert(
        11,
        Alt(Seq(vec![42, 31]).into(), Seq(vec![42, 11, 31]).into()),
    );
    let rule0 = rules.get(&0).unwrap();
    let part2 = messages
        .iter()
        .filter(|&message| match_length(rule0, &rules, message) == Some(message.len()))
        .count();
    dbg!(part2);
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

fn match_length(rule: &Rule, rules: &HashMap<usize, Rule>, input: &[u8]) -> Option<usize> {
    match rule {
        Char(c) => {
            if input.get(0) == Some(c) {
                Some(1)
            } else {
                None
            }
        }
        Seq(sequence) => {
            let mut starting = 0;
            for i in sequence {
                if let Some(l) = match_length(
                    rules.get(i).expect("nonexistent rule"),
                    rules,
                    &input[starting..],
                ) {
                    starting += l;
                } else {
                    // at least one of the sequence elements did not match
                    return None;
                }
            }
            Some(starting)
        }
        Alt(left, right) => {
            match_length(left, rules, input).or_else(|| match_length(right, rules, input))
        }
    }
}
