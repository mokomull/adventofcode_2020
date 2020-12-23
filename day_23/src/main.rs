use std::collections::VecDeque;

fn main() {
    let input = [1, 5, 6, 7, 9, 4, 8, 2, 3];
    let input = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    let mut cups: VecDeque<i32> = input.iter().cloned().collect();
    let mut current_cup = *cups.front().unwrap();

    for i in 0..100 {
        dbg!(i);
        // get the next three cups after the current cup
        while *cups.back().unwrap() != current_cup {
            let cup = cups.pop_front().unwrap();
            cups.push_back(cup);
        }

        let this_round: Vec<_> = cups.drain(..3).collect();
        let mut destination = current_cup - 1;
        while this_round.contains(&destination) {
            destination -= 1;
            if destination < 1 {
                destination = 9;
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

    dbg!(cups);
}
