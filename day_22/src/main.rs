use prelude::*;

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

    let mut decks = orig_decks.clone();
    let mut seen = HashSet::new();
    while decks.iter().all(|deck| !deck.is_empty()) {
        recursive_combat(&mut decks, &mut seen);
    }

    let part2: u32 = decks
        .iter()
        .flat_map(|d| d.iter().rev())
        .zip(1..)
        .map(|(card, i)| *card * i)
        .sum();
    dbg!(part2);
}

fn recursive_combat(
    decks: &mut Vec<VecDeque<u32>>,
    seen: &mut HashSet<Vec<VecDeque<u32>>>,
) -> usize {
    if seen.contains(decks) {
        return 0;
    }

    let round: Vec<u32> = decks
        .iter_mut()
        .map(|deck| deck.pop_front().unwrap())
        .collect();

    let should_recurse = decks
        .iter()
        .zip(round.iter())
        .all(|(deck, count)| deck.len() >= *count as usize);

    let winner = if should_recurse {
        let mut new_decks = decks
            .iter()
            .zip(&round)
            .map(|(deck, count)| deck.iter().take(*count as usize).cloned().collect())
            .collect();
        recursive_combat(&mut new_decks, seen)
    } else {
        round.iter().position_max().unwrap()
    };

    let loser = match winner {
        0 => 1,
        1 => 0,
        x => panic!("who's player {}", x),
    };

    decks[winner].push_back(round[winner]);
    decks[winner].push_back(round[loser]);

    winner
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
