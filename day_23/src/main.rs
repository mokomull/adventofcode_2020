fn main() {
    let input = [1, 5, 6, 7, 9, 4, 8, 2, 3];

    let cups = run(100, &input);

    dbg!(&cups);
    assert_eq!(&cups, &[8, 2, 5, 7, 3, 4, 9, 6]);

    let mut part2_input = input.to_vec();
    for i in 10..=1_000_000 {
        part2_input.push(i);
    }
    let part2: u64 = run(10_000_000, &part2_input)
        .drain(..2)
        .map(|i| i as u64)
        .product();
    dbg!(part2);
    assert_eq!(part2, 11498506800);
}

#[derive(Clone, Default, Debug)]
struct Cup {
    next: usize,
}

fn setup_cups(input: &[usize]) -> Vec<Cup> {
    let mut cups: Vec<Cup> = vec![Default::default(); input.len() + 1 /* because we never use 0 */];

    for (&i, &j) in input.iter().zip(input.iter().skip(1)) {
        cups[i].next = j;
    }
    cups[*input.last().unwrap()].next = input[0];

    cups
}

fn run(iterations: i32, input: &[usize]) -> Vec<usize> {
    let mut current_cup = input[0];
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut cups = setup_cups(input);

    for _ in 0..iterations {
        // splice out the next three after current_cup
        // "range" is the three cups we're moving this round, inclusive
        let beginning_of_range = cups[current_cup].next;
        let mut end_of_range = current_cup;
        let mut this_round = Vec::with_capacity(3);
        for _ in 0..3 {
            end_of_range = cups[end_of_range].next;
            this_round.push(end_of_range);
        }
        let next_after_this_round = cups[end_of_range].next;
        cups[current_cup].next = next_after_this_round;

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

        // now put the range back in between destination and its successor
        let next_after_destination = cups[destination].next;
        cups[destination].next = beginning_of_range;
        cups[end_of_range].next = next_after_destination;

        current_cup = cups[current_cup].next;
    }

    let mut ret = Vec::new();
    let mut this = 1;
    while cups[this].next != 1 {
        ret.push(cups[this].next);
        this = cups[this].next;
    }
    ret
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::main();
    }
}
