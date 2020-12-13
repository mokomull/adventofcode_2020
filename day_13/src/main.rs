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

    let part2 = (0..)
        .filter(|t| {
            buses.iter().enumerate().all(|(dt, &bus)| match bus {
                None => true,
                Some(bus_id) => (t + dt as u32) % bus_id == 0,
            })
        })
        .next()
        .expect("no suitable timestamp was found");
    dbg!(part2);
}
