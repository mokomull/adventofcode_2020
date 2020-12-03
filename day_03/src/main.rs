use std::io::BufRead;

fn main() {
    do_main("inputs/day_03.txt");
}

enum Cell {
    Empty,
    Tree,
}

fn do_main(filename: &str) {
    use Cell::*;

    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut map: Vec<Vec<Cell>> = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let line = line.expect("could not read line");
        let row = line
            .chars()
            .map(|c| match c {
                '#' => Tree,
                '.' => Empty,
                x => panic!("unexpected character: {:?}", x),
            })
            .collect();
        map.push(row);
    }

    let (mut row, mut col) = (0, 0);
    let mut part1 = 0;
    while row < map.len() {
        if let Tree = map[row][col % map[0].len()] {
            part1 += 1;
        }
        row += 1;
        col += 3;
    }
    dbg!(part1);
}
