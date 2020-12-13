use prelude::*;

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

    let part2 = (0u64..)
        .filter(|t| {
            buses.iter().enumerate().all(|(dt, &bus)| match bus {
                None => true,
                Some(bus_id) => (t + dt as u64) % bus_id as u64 == 0,
            })
        })
        .next()
        .expect("no suitable timestamp was found");
    dbg!(part2);
}

fn extended_euclidean(a: i64, b: i64) -> (i64, i64) {
    extended_euclidean_inner(a, b, 1, 0, 0, 1)
}

fn extended_euclidean_inner(a: i64, b: i64, s0: i64, s1: i64, t0: i64, t1: i64) -> (i64, i64) {
    if b == 0 {
        assert_eq!(a, 1); // this algorithm makes no sense if gcd(orig_a, orig_b) != 1
        return (s0, t0);
    }

    let q = a / b;
    extended_euclidean_inner(b, a - q * b, s1, s0 - q * s1, t1, t0 - q * t1)
}

#[cfg(test)]
mod test {
    #[test]
    fn euclidean() {
        // gcd(240, 23) == 1
        // 7 * 240 + -73 * 23 == 1
        assert_eq!(super::extended_euclidean(240, 23), (7, -73));
    }
}
