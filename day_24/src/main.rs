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
}

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
}
