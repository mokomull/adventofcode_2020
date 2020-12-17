use prelude::*;

fn main() {
    do_main("inputs/day_17.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<bool>> = read_lines_from_file(filename)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut board = HashSet::new();
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                board.insert((0, i as isize, j as isize));
            }
        }
    }

    for _ in 0..6 {
        step(&mut board);
    }

    let part1 = board.len();
    dbg!(part1);
    assert_eq!(part1, 382);

    let mut board = HashSet::new();
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell {
                board.insert((0, 0, i as isize, j as isize));
            }
        }
    }
    for _ in 0..6 {
        step_4d(&mut board);
    }

    let part2 = board.len();
    dbg!(part2);
    assert_eq!(part2, 2552);
}

fn step(board: &mut HashSet<(isize, isize, isize)>) {
    let mut new_board = board.clone();

    let (min_i, max_i) = board
        .iter()
        .map(|(i, _, _)| *i)
        .minmax()
        .into_option()
        .unwrap();
    let (min_j, max_j) = board
        .iter()
        .map(|(_, j, _)| *j)
        .minmax()
        .into_option()
        .unwrap();
    let (min_k, max_k) = board
        .iter()
        .map(|(_, _, k)| *k)
        .minmax()
        .into_option()
        .unwrap();

    for i in (min_i - 1)..=(max_i + 1) {
        for j in (min_j - 1)..=(max_j + 1) {
            for k in (min_k - 1)..=(max_k + 1) {
                let mut neighbors = 0;

                for coords in vec![
                    [i - 1, i, i + 1].iter(),
                    [j - 1, j, j + 1].iter(),
                    [k - 1, k, k + 1].iter(),
                ]
                .drain(..)
                .multi_cartesian_product()
                {
                    let target = (*coords[0], *coords[1], *coords[2]);
                    if target != (i, j, k) && board.contains(&target) {
                        neighbors += 1;
                    }
                }

                match board.contains(&(i, j, k)) {
                    false => {
                        if neighbors == 3 {
                            new_board.insert((i, j, k));
                        }
                    }
                    true => {
                        if !(2_i32..=3_i32).contains(&neighbors) {
                            new_board.remove(&(i, j, k));
                        }
                    }
                }
            }
        }
    }

    std::mem::swap(board, &mut new_board);
}

fn step_4d(board: &mut HashSet<(isize, isize, isize, isize)>) {
    let mut new_board = board.clone();

    let (min_i, max_i) = board
        .iter()
        .map(|(i, _, _, _)| *i)
        .minmax()
        .into_option()
        .unwrap();
    let (min_j, max_j) = board
        .iter()
        .map(|(_, j, _, _)| *j)
        .minmax()
        .into_option()
        .unwrap();
    let (min_k, max_k) = board
        .iter()
        .map(|(_, _, k, _)| *k)
        .minmax()
        .into_option()
        .unwrap();
    let (min_w, max_w) = board
        .iter()
        .map(|(_, _, _, w)| *w)
        .minmax()
        .into_option()
        .unwrap();

    for i in (min_i - 1)..=(max_i + 1) {
        for j in (min_j - 1)..=(max_j + 1) {
            for k in (min_k - 1)..=(max_k + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    let mut neighbors = 0;

                    for coords in vec![
                        [i - 1, i, i + 1].iter(),
                        [j - 1, j, j + 1].iter(),
                        [k - 1, k, k + 1].iter(),
                        [w - 1, w, w + 1].iter(),
                    ]
                    .drain(..)
                    .multi_cartesian_product()
                    {
                        let target = (*coords[0], *coords[1], *coords[2], *coords[3]);
                        if target != (i, j, k, w) && board.contains(&target) {
                            neighbors += 1;
                        }
                    }

                    match board.contains(&(i, j, k, w)) {
                        false => {
                            if neighbors == 3 {
                                new_board.insert((i, j, k, w));
                            }
                        }
                        true => {
                            if !(2_i32..=3_i32).contains(&neighbors) {
                                new_board.remove(&(i, j, k, w));
                            }
                        }
                    }
                }
            }
        }
    }

    std::mem::swap(board, &mut new_board);
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_17.txt");
    }
}
