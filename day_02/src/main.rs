use std::io::BufRead;

use itertools::Itertools;

fn main() {
    do_main("inputs/day_02.txt");
}

#[derive(Debug)]
struct Password {
    low: usize,
    high: usize,
    c: char,
    password: String,
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut passwords = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            let pieces = line.trim().split(&['-', ' ', ':'][..]).collect::<Vec<_>>();
            passwords.push(Password {
                low: pieces[0].parse().unwrap(),
                high: pieces[1].parse().unwrap(),
                c: pieces[2].chars().next().unwrap(),
                password: pieces[4].to_owned(),
            })
        }
    }

    let part1 = passwords
        .iter()
        .filter(|&password| {
            let matching_chars = password
                .password
                .chars()
                .filter(|&c| c == password.c)
                .count();
            matching_chars >= password.low && matching_chars <= password.high
        })
        .count();
    println!("part 1: {}", part1);
}
