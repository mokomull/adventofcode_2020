use prelude::*;
use std::collections::VecDeque;

fn main() {
    do_main("inputs/day_11.txt");
}

fn do_main(filename: &str) {
    let orig_input: Vec<Vec<Seat>> = read_lines_from_file(filename)
        .map(|line| line.chars().map(Seat::from).collect())
        .collect();

    let mut input = orig_input.clone();
    loop {
        let mut new_seats = input.clone();

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let mut occupied = 0;

                if i > 0 && input[i - 1][j] == Seat::Occupied {
                    occupied += 1;
                }

                if i > 0 && j > 0 && input[i - 1][j - 1] == Seat::Occupied {
                    occupied += 1;
                }

                if j > 0 && input[i][j - 1] == Seat::Occupied {
                    occupied += 1;
                }

                if i + 1 < input.len()
                    && j + 1 < input[i].len()
                    && input[i + 1][j + 1] == Seat::Occupied
                {
                    occupied += 1;
                }

                if i + 1 < input.len() && input[i + 1][j] == Seat::Occupied {
                    occupied += 1;
                }

                if j + 1 < input[i].len() && input[i][j + 1] == Seat::Occupied {
                    occupied += 1;
                }

                if i > 0 && j + 1 < input[i].len() && input[i - 1][j + 1] == Seat::Occupied {
                    occupied += 1;
                }

                if i + 1 < input.len() && j > 0 && input[i + 1][j - 1] == Seat::Occupied {
                    occupied += 1;
                }

                match input[i][j] {
                    Seat::Occupied => {
                        if occupied >= 4 {
                            new_seats[i][j] = Seat::Empty;
                        }
                    }
                    Seat::Empty => {
                        if occupied == 0 {
                            new_seats[i][j] = Seat::Occupied;
                        }
                    }
                    Seat::Floor => (), /* unchanged */
                }
            }
        }

        if input == new_seats {
            break;
        }
        input = new_seats;
    }

    let part1 = input
        .iter()
        .map(|row| row.iter().filter(|&seat| seat == &Seat::Occupied).count())
        .sum::<usize>();
    dbg!(part1);

    let mut input = orig_input.clone();
    loop {
        let mut new_seats = input.clone();

        for i in 0..input.len() {
            for j in 0..input[i].len() {
                let mut occupied = 0;

                for (dx, dy) in &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, 1),
                    (1, 1),
                    (1, 0),
                    (1, -1),
                    (0, -1),
                ] {
                    let mut x = i as isize + dx;
                    let mut y = j as isize + dy;

                    while x >= 0
                        && x < input.len() as isize
                        && y >= 0
                        && y < input[i].len() as isize
                    {
                        if input[x as usize][y as usize] == Seat::Occupied {
                            occupied += 1;
                            break;
                        }

                        if input[x as usize][y as usize] == Seat::Empty {
                            break;
                        }

                        x += dx;
                        y += dy;
                    }
                }

                match input[i][j] {
                    Seat::Occupied => {
                        if occupied >= 5 {
                            new_seats[i][j] = Seat::Empty;
                        }
                    }
                    Seat::Empty => {
                        if occupied == 0 {
                            new_seats[i][j] = Seat::Occupied;
                        }
                    }
                    Seat::Floor => (), /* unchanged */
                }
            }
        }

        if input == new_seats {
            break;
        }
        input = new_seats;
    }

    let part2 = input
        .iter()
        .map(|row| row.iter().filter(|&seat| seat == &Seat::Occupied).count())
        .sum::<usize>();
    dbg!(part2);
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(input: char) -> Seat {
        match input {
            'L' => Seat::Empty,
            '.' => Seat::Floor,
            '#' => Seat::Occupied,
            x => panic!("unexpected input: {}", x),
        }
    }
}
