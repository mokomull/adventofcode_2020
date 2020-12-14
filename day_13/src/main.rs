use prelude::*;

use num::bigint::BigInt;

fn main() {
    do_main("inputs/day_13.txt");
}

fn do_main(filename: &str) {
    let mut lines = read_lines_from_file(filename);

    let timestamp: u32 = lines
        .next()
        .expect("input was empty")
        .parse()
        .expect("timestamp was not an integer");
    let buses: Vec<Option<u32>> = lines
        .next()
        .expect("list of buses is empty")
        .split(",")
        .map(|bus| match bus {
            "x" => None,
            frequency => Some(frequency.parse().expect("bus was not an integer")),
        })
        .collect();

    let mut chosen = None;

    'out: for t in timestamp.. {
        for b in &buses {
            if let Some(bus) = b {
                if t % bus == 0 {
                    chosen = Some((t, *bus));
                    break 'out;
                }
            }
        }
    }

    let (t, bus) = chosen.expect("no suitable bus was found");
    let part1 = (t - timestamp) * bus;
    dbg!(part1);
    assert_eq!(part1, 4938);

    let (part2, _bus_product) = buses
        .iter()
        .cloned()
        .enumerate()
        .filter_map(|(idx, bus)| {
            // fix up types so I can pretend everything is a BigInt
            //
            // and fix up indices -- the problem gives t + idx == 0 (mod bus_id),
            // which means t is congruent to -idx (mod bus_id).  Represent with
            // (remainder, modulus) tuples.
            match bus {
                None => None,
                Some(bus_id) => {
                    let bus_id: BigInt = bus_id.into();
                    let idx: BigInt = idx.into();
                    let remainder = (-idx).modpow(&1.into(), &bus_id);
                    Some((remainder, bus_id))
                }
            }
        })
        .fold1(|(a0, n0), (a1, n1)| {
            // solve Chinese Remainder Theorem for the two buses:
            //   t = a0 (mod n0)
            //   t = a1 (mod n1)
            // and continue with the equivalent t mod (n0 * n1)
            let (m0, _m1) = extended_euclidean(n0.clone(), n1.clone());
            let t = &a0 + (a1 - &a0) * m0 * &n0;
            let new_n = n0 * n1;
            (t.modpow(&1.into(), &new_n), new_n)
        })
        .expect("there were no buses");
    dbg!(&part2);
    assert_eq!(part2, 230903629977901i64.into());
}

fn extended_euclidean(a: BigInt, b: BigInt) -> (BigInt, BigInt) {
    extended_euclidean_inner(a, b, 1.into(), 0.into(), 0.into(), 1.into())
}

fn extended_euclidean_inner(
    a: BigInt,
    b: BigInt,
    s0: BigInt,
    s1: BigInt,
    t0: BigInt,
    t1: BigInt,
) -> (BigInt, BigInt) {
    if b == 0.into() {
        assert_eq!(a, 1.into()); // this algorithm makes no sense if gcd(orig_a, orig_b) != 1
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
    fn euclidean() {
        // gcd(240, 23) == 1
        // 7 * 240 + -73 * 23 == 1
        assert_eq!(
            super::extended_euclidean(240.into(), 23.into()),
            (7.into(), (-73).into())
        );
    }

    #[test]
    fn main() {
        super::do_main("../inputs/day_13.txt")
    }
}
