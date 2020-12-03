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

    let part1 = count_hits(&map, 3, 1);
    dbg!(part1);
    assert_eq!(part1, 205);
}

fn count_hits(map: &Vec<Vec<Cell>>, right: usize, down: usize) -> usize {
    assert!(down > 0);
    let (mut row, mut col) = (0, 0);
    let mut res = 0;
    while row < map.len() {
        if let Cell::Tree = map[row][col % map[0].len()] {
            res += 1;
        }
        row += down;
        col += right;
    }
    res
}
