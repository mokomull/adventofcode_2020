use prelude::*;

use Rule::*;

fn main() {
    do_main("inputs/day_19.txt");
}

fn do_main(filename: &str) {
    let (rules, messages) = parse_lines(&mut read_lines_from_file(filename));
}

#[derive(Debug)]
enum Rule {
    Char(u8),
    Seq(Vec<usize>),
    Alt(Box<Rule>, Box<Rule>),
}

fn parse_lines(
    mut input: &mut impl Iterator<Item = String>,
) -> (HashMap<usize, Rule>, Vec<String>) {
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

    let messages = input.collect();
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
