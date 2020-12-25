use prelude::*;

fn main() {
    do_main("inputs/day_25.txt");
}

fn do_main(filename: &str) {
    let keys = read_lines_from_file(filename)
        .map(|l| l.parse::<u64>().expect("not an integer"))
        .collect_vec();

    let private_keys = keys
        .iter()
        .map(|&pubkey| get_private_key(pubkey))
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

fn get_private_key(pubkey: u64) -> u64 {
    for i in 0.. {
        if transform(7, i) == pubkey {
            return i;
        }
    }
    panic!("could not find a suitable public key");
}

#[cfg(test)]
mod test {
    #[test]
    fn transform() {
        assert_eq!(super::transform(7, 8), 5764801);
    }

    #[test]
    fn get_private_key() {
        assert_eq!(super::get_private_key(5764801), 8);
    }
}
