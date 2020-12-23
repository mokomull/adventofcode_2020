use std::collections::VecDeque;

fn main() {
    let input = [1, 5, 6, 7, 9, 4, 8, 2, 3];

    let mut cups: VecDeque<i32> = input.iter().cloned().collect();
    run(100, &mut cups);

    dbg!(cups);

    let mut cups: VecDeque<i32> = input.iter().cloned().collect();
    for i in 10..=1_000_000 {
        cups.push_back(i);
    }
    run(10_000_000, &mut cups);

    while *cups.back().unwrap() != 1 {
        let cup = cups.pop_front().unwrap();
        cups.push_back(cup);
    }
    let part2: i64 = cups.drain(..2).map(|i| i as i64).product();
    dbg!(part2);
}

fn run(iterations: i32, cups: &mut VecDeque<i32>) {
    let mut current_cup = *cups.front().unwrap();
    let min = *cups.iter().min().unwrap();
    let max = *cups.iter().max().unwrap();

    for i in 0..iterations {
        dbg!(i);
        // get the next three cups after the current cup
        while *cups.back().unwrap() != current_cup {
            let cup = cups.pop_front().unwrap();
            cups.push_back(cup);
        }

        let this_round: Vec<_> = cups.drain(..3).collect();
        let mut destination = current_cup - 1;
        if destination < min {
            destination = max;
        }
        while this_round.contains(&destination) {
            destination -= 1;
            if destination < min {
                destination = max;
            }
        }

        // now make sure the destination is at the end
        while *cups.back().unwrap() != destination {
            let cup = cups.pop_front().unwrap();
            cups.push_back(cup);
        }

        for cup in this_round {
            cups.push_back(cup);
        }

        // find the current cup again
        while *cups.back().unwrap() != current_cup {
            let cup = cups.pop_front().unwrap();
            cups.push_back(cup);
        }

        current_cup = *cups.front().unwrap();
    }
}
