use prelude::*;

use Direction::*;

fn main() {
    do_main("inputs/day_24.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<Direction>> = read_lines_from_file(filename)
        .map(|line| parse_line(line.as_bytes()))
        .collect();
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
