use prelude::*;

fn main() {
    do_main("inputs/day_20.txt");
}

fn do_main(filename: &str) {
    let tiles = parse_tiles(read_lines_from_file(filename));

    let mut edge_counts = HashMap::<usize, usize>::new();
    for (t1, t2) in tiles.iter().tuple_combinations() {
        for (e1, e2) in [t1.left(), t1.right(), t1.top(), t1.bottom()]
            .iter()
            .zip([t2.left(), t2.right(), t2.top(), t2.bottom()].iter())
        {
            if e1 == e2 {
                *edge_counts.entry(t1.id).or_default() += 1;
                *edge_counts.entry(t2.id).or_default() += 1;
            }
            let mut e2 = e2.clone();
            e2.reverse();
            if e1 == &e2 {
                *edge_counts.entry(t1.id).or_default() += 1;
                *edge_counts.entry(t2.id).or_default() += 1;
            }
        }
    }
    dbg!(edge_counts);
}

#[derive(Debug)]
struct Tile {
    id: usize,
    image: Vec<Vec<bool>>,
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
