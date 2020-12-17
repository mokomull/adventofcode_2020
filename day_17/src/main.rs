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
                board.insert(vec![0, i as isize, j as isize]);
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
                board.insert(vec![0, 0, i as isize, j as isize]);
            }
        }
    }
    for _ in 0..6 {
        step(&mut board);
    }

    let part2 = board.len();
    dbg!(part2);
    assert_eq!(part2, 2552);
}

fn step(board: &mut HashSet<Vec<isize>>) {
    let mut new_board = board.clone();

    let dimension = board.iter().next().expect("board was empty!").len();
    let coordinate_ranges = (0..dimension).into_iter().map(|d| {
        let (min, max) = board
            .iter()
            .map(|coord| coord[d])
            .minmax()
            .into_option()
            .unwrap();
        (min - 1)..=(max + 1)
    });

    for coord in coordinate_ranges.multi_cartesian_product() {
        let mut neighbors = 0;

        for neighbor in coord
            .iter()
            .map(|&c| (c - 1)..=(c + 1))
            .multi_cartesian_product()
        {
            if neighbor != coord && board.contains(&neighbor) {
                neighbors += 1;
            }
        }

        match board.contains(&coord) {
            false => {
                if neighbors == 3 {
                    new_board.insert(coord);
                }
            }
            true => {
                if !(2_i32..=3_i32).contains(&neighbors) {
                    new_board.remove(&coord);
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
