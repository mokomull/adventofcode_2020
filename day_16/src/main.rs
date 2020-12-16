use std::ops::RangeInclusive;

use prelude::*;

fn main() {
    do_main("inputs/day_16.txt");
}

fn do_main(filename: &str) {
    let input: TicketData = read_lines_from_file(filename).into();
}

type FieldRange = Vec<RangeInclusive<i64>>;

struct TicketData {
    fields: Vec<FieldRange>,
    my_ticket: Vec<i64>,
    nearby_tickets: Vec<Vec<i64>>,
}

impl<I, S> From<I> for TicketData
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn from(mut input: I) -> TicketData {
        let mut fields = Vec::new();

        for line in &mut input {
            let line = line.as_ref();

            if line == "" {
                break;
            }

            let mut this_fields = FieldRange::new();

            for range in line.split_whitespace().skip(1).step_by(2) {
                let mut ends = range.split("-");
                let begin = ends
                    .next()
                    .expect("no begin")
                    .parse()
                    .expect("not an integer");
                let end = ends
                    .next()
                    .expect("no end")
                    .parse()
                    .expect("not an integer");
                this_fields.push(RangeInclusive::new(begin, end));
            }

            fields.push(this_fields)
        }

        // skip over "your ticket:"
        let _ = input.next();

        let my_ticket = input
            .next()
            .expect("your ticket was not found")
            .as_ref()
            .split(",")
            .map(|s| s.parse().expect("not an integer"))
            .collect();

        // skip over blank line and "nearby tickets:"
        let _ = input.next();
        let _ = input.next();

        let nearby_tickets = input
            .map(|s| {
                s.as_ref()
                    .split(",")
                    .map(|s| s.parse().expect("not an integer"))
                    .collect()
            })
            .collect();

        TicketData {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }
}
