use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    do_main("inputs/day_06.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut seen = HashSet::new();
    let mut sum_of_counts = 0;

    for line in std::io::BufReader::new(file)
        .lines()
        .chain(std::iter::once(Ok("".to_owned())))
    {
        let line = line.expect("could not read line");

        if line == "" {
            sum_of_counts += seen.iter().count();
            seen = HashSet::new();
        } else {
            for c in line.chars() {
                seen.insert(c);
            }
        }
    }

    dbg!(sum_of_counts);
}
