use std::ops::RangeInclusive;

use prelude::*;

fn main() {
    do_main("inputs/day_16.txt");
}

fn do_main(filename: &str) {
    let input: TicketData = read_lines_from_file(filename).into();

    let part1: i64 = input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|&field| {
                    // none of the fields in any of the ranges can match the value in the ticket
                    // we're looking at.
                    !can_be_valid_field(*field, &input.fields)
                })
                .sum::<i64>()
        })
        .sum();
    dbg!(part1);

    let valid_tickets: Vec<&Vec<i64>> = input
        .nearby_tickets
        .iter()
        .filter(|&ticket| {
            ticket
                .iter()
                .all(|field| can_be_valid_field(*field, &input.fields))
        })
        .collect();

    let mut field_order = None;

    for order in input.fields.iter().permutations(input.fields.len()) {
        dbg!(&order);
        if valid_tickets.iter().all(|&ticket| {
            ticket
                .iter()
                .zip(order.iter())
                .all(|(ticket_value, ranges)| can_be_valid_field(*ticket_value, &[ranges.to_vec()]))
        }) {
            field_order = Some(order);
            break;
        }
    }
}

fn can_be_valid_field(field: i64, fields: &[FieldRange]) -> bool {
    fields.iter().any(|possible_field| {
        possible_field
            .iter()
            .any(|possible_values| possible_values.contains(&field))
    })
}

type FieldRange = Vec<RangeInclusive<i64>>;

#[derive(Debug)]
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

            for range in line
                .split(": ")
                .skip(1)
                .next()
                .expect("field values were not given")
                .split_whitespace()
                .step_by(2)
            {
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
