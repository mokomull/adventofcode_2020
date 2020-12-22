use prelude::*;

use std::collections::VecDeque;

fn main() {
    do_main("inputs/day_22.txt");
}

fn do_main(filename: &str) {
    let mut decks =
        parse_lines(&mut read_lines_from_file(filename).chain(std::iter::once("".into())));

    while decks.iter().all(|deck| !deck.is_empty()) {
        let round: Vec<u32> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();
        let winner = round.iter().position_max().unwrap();
        let loser = match winner {
            0 => 1,
            1 => 0,
            x => panic!("who's player {}", x),
        };

        decks[winner].push_back(round[winner]);
        decks[winner].push_back(round[loser]);
    }

    let part1: u32 = decks
        .iter()
        .flat_map(|d| d.iter().rev())
        .zip(1..)
        .map(|(card, i)| *card * i)
        .sum();
    dbg!(part1);
}

fn parse_lines<I>(input: I) -> Vec<VecDeque<u32>>
where
    I: Iterator<Item = String>,
{
    let mut ret = Vec::new();

    let mut this_deck = VecDeque::new();

    for line in input {
        if line.starts_with("Player ") {
            continue;
        }
        if line.is_empty() {
            ret.push(this_deck);
            this_deck = VecDeque::new();
            continue;
        }

        this_deck.push_back(line.parse().expect("not an integer"));
    }

    ret
}
