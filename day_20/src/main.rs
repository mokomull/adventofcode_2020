use prelude::*;

use Edge::*;

fn main() {
    do_main("inputs/day_20.txt");
}

fn do_main(filename: &str) {
    let tiles = parse_tiles(read_lines_from_file(filename));

    let mut edge_counts = HashMap::<(usize, Edge), usize>::new();
    for (t1, t2) in tiles.iter().tuple_combinations() {
        for (e1, e2) in [
            (Left, t1.left()),
            (Right, t1.right()),
            (Top, t1.top()),
            (Bottom, t1.bottom()),
        ]
        .iter()
        .zip(
            [
                (Left, t2.left()),
                (Right, t2.right()),
                (Top, t2.top()),
                (Bottom, t2.bottom()),
            ]
            .iter(),
        ) {
            if e1.1 == e2.1 {
                *edge_counts.entry((t1.id, e1.0)).or_default() += 1;
                *edge_counts.entry((t2.id, e2.0)).or_default() += 1;
            }
            let mut reversed = e2.1.clone();
            reversed.reverse();
            if e1.1 == reversed {
                *edge_counts.entry((t1.id, e1.0)).or_default() += 1;
                *edge_counts.entry((t2.id, e2.0)).or_default() += 1;
            }
        }
    }
    dbg!(edge_counts.len());
}

#[derive(Debug)]
struct Tile {
    id: usize,
    image: Vec<Vec<bool>>,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

fn parse_tiles<I>(input: I) -> Vec<Tile>
where
    I: IntoIterator<Item = String>,
{
    let mut ret = Vec::new();

    let mut id = None;
    let mut rows = Vec::new();
    for line in input {
        if line.is_empty() {
            ret.push(Tile {
                id: id.unwrap(),
                image: rows,
            });
            id = None;
            rows = Vec::new();
            continue;
        }

        if line.starts_with("Tile ") {
            id = Some(line.split(&[' ', ':'][..]).nth(1).unwrap().parse().unwrap());
        } else {
            rows.push(line.chars().map(|c| c == '#').collect())
        }
    }

    ret
}

impl Tile {
    fn top(&self) -> Vec<bool> {
        self.image[0].clone()
    }

    fn left(&self) -> Vec<bool> {
        self.image.iter().map(|row| row[0]).collect()
    }

    fn right(&self) -> Vec<bool> {
        self.image
            .iter()
            .map(|row| row.last().unwrap())
            .cloned()
            .collect()
    }

    fn bottom(&self) -> Vec<bool> {
        self.image.last().unwrap().clone()
    }
}
