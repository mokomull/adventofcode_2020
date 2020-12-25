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
    let part1 = transform(keys[0], private_keys[1]);
    dbg!(part1);
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
    let m = ((20201227 as f64).sqrt() + 1.0) as u64;

    let table = (0..=m)
        .map(|j| (transform(7, j), j))
        .collect::<HashMap<_, _>>();

    let (_q1, q2) = extended_euclidean(20201227, transform(7, m));
    let inverse = q2.rem_euclid(20201227);

    let mut y = pubkey;
    for i in 0..m {
        if let Some(j) = table.get(&y) {
            return i * m + j;
        }
        y *= inverse;
        y %= 20201227;
    }
    panic!("could not find a suitable public key");
}

fn extended_euclidean(a: u64, b: u64) -> (u64, u64) {
    extended_euclidean_inner(a, b, 1, 0, 0, 1)
}

fn extended_euclidean_inner(a: u64, b: u64, s0: u64, s1: u64, t0: u64, t1: u64) -> (u64, u64) {
    if b == 0 {
        assert_eq!(a, 1); // this algorithm makes no sense if gcd(orig_a, orig_b) != 1
        return (s0, t0);
    }

    let q = &a / &b;
    extended_euclidean_inner(
        b.clone(),
        a - &q * b,
        s1.clone(),
        s0 - &q * s1,
        t1.clone(),
        t0 - q * t1,
    )
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
