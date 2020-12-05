use std::io::BufRead;

fn main() {
    do_main("inputs/day_05.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let part1 = std::io::BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.expect("could not read line");
            seat_from_directions(&line)
        })
        .map(|(row, column)| row * column)
        .max();
    dbg!(part1);
}

fn seat_from_directions(directions: &str) -> (u32, u32) {
    let row = directions
        .chars()
        .take(7)
        .map(|c| match c {
            'F' => '0',
            'B' => '1',
            x => panic!("unexpected front/back: {}", x),
        })
        .collect::<String>();
    let column = directions
        .chars()
        .skip(7)
        .take(3)
        .map(|c| match c {
            'L' => '0',
            'R' => '1',
            x => panic!("unexpected left/right: {}", x),
        })
        .collect::<String>();
    (
        u32::from_str_radix(&row, 2).expect("could not parse row"),
        u32::from_str_radix(&column, 2).expect("could not parse column"),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(seat_from_directions("BFFFBBFRRR"), (70, 7));
        assert_eq!(seat_from_directions("FFFBBBFRRR"), (14, 7));
        assert_eq!(seat_from_directions("BBFFBBFRLL"), (102, 4));
    }
}
