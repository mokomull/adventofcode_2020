use std::io::{BufRead, BufReader};

pub use std::collections::{HashMap, HashSet};

pub fn read_lines_from_file<P: AsRef<std::path::Path>>(
    filename: P,
) -> impl Iterator<Item = String> {
    let file = std::fs::File::open(filename).expect("could not open the input");
    BufReader::new(file)
        .lines()
        .map(|line| line.expect("could not read a line"))
}
