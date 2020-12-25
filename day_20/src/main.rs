use prelude::*;

use Edge::*;

fn main() {
    do_main("inputs/day_20.txt");
}

fn do_main(filename: &str) {
    let tiles = parse_tiles(read_lines_from_file(filename));

    let mut edge_matches = HashMap::<(usize, Edge), (usize, Edge, bool)>::new();
    for (t1, t2) in tiles.iter().tuple_combinations() {
        for (e1, e2) in [
            (Left, t1.left()),
            (Right, t1.right()),
            (Top, t1.top()),
            (Bottom, t1.bottom()),
        ]
        .iter()
        .cartesian_product(
            [
                (Left, t2.left()),
                (Right, t2.right()),
                (Top, t2.top()),
                (Bottom, t2.bottom()),
            ]
            .iter(),
        ) {
            if e1.1 == e2.1 {
                edge_matches.insert((t1.id, e1.0), (t2.id, e2.0, false));
                edge_matches.insert((t2.id, e2.0), (t1.id, e1.0, false));
            }
            let mut reversed = e2.1.clone();
            reversed.reverse();
            if e1.1 == reversed {
                edge_matches.insert((t1.id, e1.0), (t2.id, e2.0, true));
                edge_matches.insert((t2.id, e2.0), (t1.id, e1.0, true));
            }
        }
    }

    let mut part1 = 1;
    let mut corner = None;
    for tile in &tiles {
        let matching_edges = [Left, Right, Top, Bottom]
            .iter()
            .filter(|&edge| edge_matches.contains_key(&(tile.id, *edge)))
            .count();
        if matching_edges == 2 {
            // this must be a corner if only two edges are in the set
            part1 *= tile.id;
            corner = Some(tile.id);
        }
    }
    dbg!(part1);

    let by_id = tiles.iter().map(|t| (t.id, t)).collect::<HashMap<_, _>>();

    // invariant: edge is always the edge of the input data that will end up on the LEFT-hand edge
    // of the next tile in the reconstructed image.
    let mut next_tile = corner.unwrap();
    let mut edge;
    let mut flipped = false;

    // figure out which edge of the corner tile we picked will be the "left"
    edge = match [Left, Top, Right, Bottom]
        .iter()
        .filter(|&edge| edge_matches.contains_key(&(next_tile, *edge)))
        .collect_vec()
        .as_slice()
    {
        [Left, Top] => Right,
        [Top, Right] => Bottom,
        [Right, Bottom] => Left,
        [Left, Bottom] => Top,
        x => panic!("unexpected set of edges for the corner: {:?}", x),
    };

    // id, "left" edge, and flipped-ness, to be used after we've finished copying a whole row
    let (mut next_row, mut next_row_edge, mut next_row_flipped) = {
        let bottom_edge = match edge {
            Left => Bottom,
            Top => Left,
            Right => Top,
            Bottom => Right,
        };
        edge_matches[&(next_tile, bottom_edge)]
    };

    let mut reconstructed_image = vec![vec![]; by_id[&next_tile].image.len() - 2];
    loop {
        // copy this tile to the image (excluding its borders)
        let mut tile = by_id[&next_tile].clone();
        let rotations = match edge {
            Left => 0,
            Top => 3,
            Right => 2,
            Bottom => 1,
        };
        for _ in 0..rotations {
            tile.rotate_clockwise();
        }
        if flipped {
            tile.flip_vertically();
        }

        let start_row = reconstructed_image.len() - (tile.image.len() - 2);
        for (dest, src) in reconstructed_image
            .iter_mut()
            .skip(start_row)
            .zip(tile.image.drain(1..))
        {
            for pixel in &src[1..src.len() - 1] {
                dest.push(*pixel);
            }
        }

        // figure out what the next tile to the right should be, and its orientation

        // if that didn't work, start on the next row

        // and if there isn't a next row, quit.
    }
}

#[derive(Clone, Debug)]
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

    fn rotate_clockwise(&mut self) {
        let mut new_image = Vec::new();

        for input_column in 0..self.image[0].len() {
            let mut new_row: Vec<bool> = self.image.iter().map(|row| row[input_column]).collect();

            // that gives the pixels in the column from the top down -- but the left-hand side needs
            // to be what was originally on the *bottom*
            new_row.reverse();

            new_image.push(new_row);
        }

        self.image = new_image;
    }

    fn flip_vertically(&mut self) {
        self.image.reverse();
    }
}
