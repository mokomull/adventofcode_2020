use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    do_main("inputs/day_06.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut groups = Vec::new();
    let mut this = Vec::new();

    for line in std::io::BufReader::new(file)
        .lines()
        .chain(std::iter::once(Ok("".to_owned())))
    {
        let line = line.expect("could not read line");

        if line == "" {
            groups.push(this);
            this = Vec::new();
        } else {
            this.push(line.chars().collect::<HashSet<_>>());
        }
    }

    let sum_of_counts: usize = groups
        .iter()
        .map(|group| {
            group
                .iter()
                .flatten()
                .collect::<HashSet<_>>()
                .iter()
                .count()
        })
        .sum();

    dbg!(sum_of_counts);
    assert_eq!(sum_of_counts, 6291);
}
