use prelude::*;

use tracing::{info, info_span};

use std::collections::VecDeque;

fn main() {
    do_main("inputs/day_22.txt");
}

fn do_main(filename: &str) {
    let orig_decks =
        parse_lines(&mut read_lines_from_file(filename).chain(std::iter::once("".into())));
    let mut decks = orig_decks.clone();

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

    tracing_subscriber::fmt::init();

    let decks = orig_decks.clone();
    let decks = recursive_combat(decks);

    let part2: u32 = decks
        .iter()
        .flat_map(|d| d.iter().rev())
        .zip(1..)
        .map(|(card, i)| *card * i)
        .sum();
    dbg!(part2);
}

fn recursive_combat(mut decks: Vec<VecDeque<u32>>) -> Vec<VecDeque<u32>> {
    let mut seen = HashSet::new();

    while decks.iter().all(|d| !d.is_empty()) {
        let span = info_span!("recursive_combat", "{:?}", decks);
        let _enter = span.enter();

        let has_been_seen = !seen.insert(decks.clone());

        let round: Vec<u32> = decks
            .iter_mut()
            .map(|deck| deck.pop_front().unwrap())
            .collect();

        let winner = if has_been_seen {
            info!("I've seen this before");
            0
        } else {
            let should_recurse = decks
                .iter()
                .zip(round.iter())
                .all(|(deck, count)| deck.len() >= *count as usize);

            if should_recurse {
                let new_decks = decks
                    .iter()
                    .zip(&round)
                    .map(|(deck, count)| deck.iter().take(*count as usize).cloned().collect())
                    .collect();
                let recursive_result = recursive_combat(new_decks);
                assert!(recursive_result.iter().any(|d| d.is_empty()));
                recursive_result
                    .iter()
                    .enumerate()
                    .filter(|&(_i, d)| !d.is_empty())
                    .map(|(i, _d)| i)
                    .next()
                    .unwrap()
            } else {
                round.iter().position_max().unwrap()
            }
        };

        info!("winner: {}", winner);

        let loser = match winner {
            0 => 1,
            1 => 0,
            x => panic!("who's player {}", x),
        };

        decks[winner].push_back(round[winner]);
        decks[winner].push_back(round[loser]);
    }

    decks
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
