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

    let part2 = passwords
        .iter()
        .filter(|&password| part2_check(password))
        .count();
    println!("part 2: {}", part2);
}

fn part2_check(password: &Password) -> bool {
    password
        .password
        .chars()
        .enumerate()
        .filter(|&(i, c)| c == password.c && (i == password.low - 1 || i == password.high - 1))
        .count()
        == 1
}

#[cfg(test)]
mod test {
    #[test]
    fn part2_check() {
        assert!(super::part2_check(&super::Password {
            low: 1,
            high: 3,
            c: 'a',
            password: "abcde".into(),
        }));
        assert!(!super::part2_check(&super::Password {
            low: 1,
            high: 3,
            c: 'b',
            password: "cdefg".into(),
        }));
        assert!(!super::part2_check(&super::Password {
            low: 2,
            high: 9,
            c: 'c',
            password: "ccccccccc".into(),
        }));
    }
}
