use prelude::*;

fn main() {
    do_main("inputs/day_20.txt");
}

fn do_main(filename: &str) {
    let tiles = parse_tiles(read_lines_from_file(filename));
}

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
        self.image.iter().map(|row| row.last().unwrap()).cloned().collect()
    }

    fn bottom(&self) -> Vec<bool> {
        self.image.last().unwrap().clone()
    }
}