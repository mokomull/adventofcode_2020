use prelude::*;

fn main() {
    do_main("inputs/day_25.txt");
}

fn do_main(filename: &str) {
    let keys = read_lines_from_file(filename)
        .map(|l| l.parse::<u64>().expect("not an integer"))
        .collect_vec();
}

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;

    for _ in 0..loop_size {
        value *= subject;
        value %= 20201227;
    }

    value
}

#[cfg(test)]
mod test {
    #[test]
    fn transform() {
        assert_eq!(super::transform(7, 8), 5764801);
    }
}
