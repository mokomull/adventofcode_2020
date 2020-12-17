use prelude::*;

fn main() {
    do_main("inputs/day_17.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<bool>> = read_lines_from_file(filename)
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let mut board = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            board.insert((0, i as isize, j as isize), *cell);
        }
    }

    for _ in 0..6 {
        step(&mut board);
    }

    let part1 = board.values().filter(|&x| *x).count();
    dbg!(part1);
}

fn step(board: &mut HashMap<(isize, isize, isize), bool>) {
    let mut new_board = board.clone();

    let (min_i, max_i) = board
        .keys()
        .map(|(i, _, _)| *i)
        .minmax()
        .into_option()
        .unwrap();
    let (min_j, max_j) = board
        .keys()
        .map(|(_, j, _)| *j)
        .minmax()
        .into_option()
        .unwrap();
    let (min_k, max_k) = board
        .keys()
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
                    if target != (i, j, k) && *board.get(&target).unwrap_or(&false) {
                        neighbors += 1;
                    }
                }

                match *board.get(&(i, j, k)).unwrap_or(&false) {
                    false => {
                        if neighbors == 3 {
                            new_board.insert((i, j, k), true);
                        }
                    }
                    true => {
                        if (2_i32..=3_i32).contains(&neighbors) {
                            new_board.insert((i, j, k), false);
                        }
                    }
                }
            }
        }
    }

    std::mem::swap(board, &mut new_board);
}
