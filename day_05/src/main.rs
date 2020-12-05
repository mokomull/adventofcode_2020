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
            let row = line
                .chars()
                .take(7)
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    x => panic!("unexpected front/back: {}", x),
                })
                .collect::<String>();
            let column = line
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
        })
        .map(|(row, column)| row * column)
        .max();
    dbg!(part1);
}
