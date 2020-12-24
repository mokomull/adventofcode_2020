use prelude::*;

use Direction::*;

fn main() {
    do_main("inputs/day_24.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<Direction>> = read_lines_from_file(filename)
        .map(|line| parse_line(line.as_bytes()))
        .collect();

    let mut black_up = HashSet::new();
    for line in input {
        let coord = line
            .iter()
            .fold((0, 0), |coord, direction| hex_move(direction, coord));
        if black_up.contains(&coord) {
            black_up.remove(&coord);
        } else {
            black_up.insert(coord);
        }
    }
    let part1 = black_up.len();
    dbg!(part1);

    for _ in 0..100 {
        step_life(&mut black_up);
    }
    let part2 = black_up.len();
    dbg!(part2);
}

// This implements a hexagonal coordinate system, with x on the horizontal, and y on the vertical,
// where the odd rows are shifted eastward a smidge:
//    -1,  2        0,  2       1,  2       2,  2
//           -1,  1       0,  1       1,  1       2, 1
//    -1,  0        0,  0       1,  0       2,  0
//           -1, -1       0, -1       1, -1
//    -1, -2        0, -2       1, -2       2, -2

fn hex_move(direction: &Direction, coord: (i32, i32)) -> (i32, i32) {
    let northorsouth_east_x_coord = if coord.1 % 2 == 0 {
        coord.0
    } else {
        coord.0 + 1
    };

    match direction {
        East => (coord.0 + 1, coord.1),
        West => (coord.0 - 1, coord.1),
        Northeast => (northorsouth_east_x_coord, coord.1 + 1),
        Northwest => (northorsouth_east_x_coord - 1, coord.1 + 1),
        Southeast => (northorsouth_east_x_coord, coord.1 - 1),
        Southwest => (northorsouth_east_x_coord - 1, coord.1 - 1),
    }
}

fn step_life(black_up: &mut HashSet<(i32, i32)>) {
    let (min_x, max_x) = black_up.iter().map(|c| c.0).minmax().into_option().unwrap();
    let (min_y, max_y) = black_up.iter().map(|c| c.1).minmax().into_option().unwrap();

    let mut new_black_up = black_up.clone();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let neighbors = black_neighbors((x, y), black_up);
            if black_up.contains(&(x, y)) {
                if neighbors == 0 || neighbors > 2 {
                    new_black_up.remove(&(x, y));
                }
            } else {
                if neighbors == 2 {
                    new_black_up.insert((x, y));
                }
            }
        }
    }
    *black_up = new_black_up;
}

fn black_neighbors(coord: (i32, i32), black_up: &HashSet<(i32, i32)>) -> usize {
    let (x, y) = coord;
    let northorsouth_east_x_coord = if coord.1 % 2 == 0 {
        coord.0
    } else {
        coord.0 + 1
    };

    [
        (northorsouth_east_x_coord, y + 1),
        (northorsouth_east_x_coord - 1, y + 1),
        (northorsouth_east_x_coord, y - 1),
        (northorsouth_east_x_coord - 1, y - 1),
        (x + 1, y),
        (x - 1, y),
    ]
    .iter()
    .filter(|&c| black_up.contains(c))
    .count()
}

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

fn parse_line(input: &[u8]) -> Vec<Direction> {
    let mut i = 0;
    let mut ret = Vec::new();

    while i < input.len() {
        match input[i] {
            b'e' => {
                ret.push(East);
                i += 1;
            }
            b'w' => {
                ret.push(West);
                i += 1;
            }
            b'n' => {
                match input[i + 1] {
                    b'e' => ret.push(Northeast),
                    b'w' => ret.push(Northwest),
                    x => panic!("wtf {} {}", x, i),
                };
                i += 2;
            }
            b's' => {
                match input[i + 1] {
                    b'e' => ret.push(Southeast),
                    b'w' => ret.push(Southwest),
                    x => panic!("wtf {} {}", x, i),
                };
                i += 2;
            }
            x => panic!("lolwtf {} {}", x, i),
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parser() {
        assert_eq!(
            parse_line(b"sesenwnenenewseeswwswswwnenewsewsw"),
            [
                Southeast, Southeast, Northwest, Northeast, Northeast, Northeast, West, Southeast,
                East, Southwest, West, Southwest, Southwest, West, Northeast, Northeast, West,
                Southeast, West, Southwest
            ]
        )
    }

    #[test]
    fn black_neighbors() {
        let black_up: HashSet<(i32, i32)> =
            [(0, 0), (1, 1), (0, 2), (1, 3)].iter().cloned().collect();
        assert_eq!(super::black_neighbors((0, 1), &black_up), 3);
        assert_eq!(super::black_neighbors((1, 2), &black_up), 3);
        assert_eq!(super::black_neighbors((-1, 1), &black_up), 2);
        assert_eq!(super::black_neighbors((-2, 1), &black_up), 0);
        assert_eq!(super::black_neighbors((0, 3), &black_up), 2);
        assert_eq!(super::black_neighbors((-1, 2), &black_up), 1);
    }
}
