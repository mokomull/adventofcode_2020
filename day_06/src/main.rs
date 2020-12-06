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

    let sum_of_intersections: usize = groups
        .iter()
        .map(|group| {
            // thanks to
            // https://www.reddit.com/r/rust/comments/5v35l6/intersection_of_more_than_two_sets/ddz06ho/
            // for this trick.

            // this clone is "expensive" but we need to initialize the storage from somewhere
            let first_group = group.iter().next().unwrap().clone();
            group
                .iter()
                .fold(first_group, |prev, this| {
                    // this clone, on the other hand, is just a clone of chars.  This one is
                    // "cheap".
                    prev.intersection(this).cloned().collect()
                })
                .iter()
                .count()
        })
        .sum();
    dbg!(sum_of_intersections);
    assert_eq!(sum_of_intersections, 3052);
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_06.txt");
    }
}
