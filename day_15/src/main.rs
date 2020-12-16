use prelude::*;

fn main() {
    let input = vec![2, 0, 1, 7, 4, 14, 18];
    let part1 = nth_spoken(&input, 2020);
    dbg!(part1);
    assert_eq!(part1, 496);

    let part2 = nth_spoken(&input, 30000000);
    dbg!(part2);
    assert_eq!(part2, 883);
}

fn nth_spoken(initial: &[i32], n: i32) -> i32 {
    let mut spoken = HashMap::new();
    let mut next_to_speak: i32 = 0;

    for (i, v) in initial.iter().enumerate() {
        let turn = (i + 1) as i32;
        match spoken.insert(*v, turn) {
            None => next_to_speak = 0,
            Some(prev) => next_to_speak = turn - prev,
        }
    }

    // Complete n-1 rounds, at which point we know what will be uttered in round n.
    for turn in (initial.len() as i32 + 1)..n {
        match spoken.insert(next_to_speak, turn) {
            None => next_to_speak = 0,
            Some(prev) => next_to_speak = turn - prev,
        }
    }

    next_to_speak as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn nth_spoken() {
        assert_eq!(super::nth_spoken(&[0, 3, 6], 10), 0);
    }

    #[test]
    fn main() {
        super::main();
    }
}
