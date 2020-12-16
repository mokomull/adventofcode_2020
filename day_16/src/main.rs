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
                    !can_be_valid_value_for_field(*field, &input.fields)
                })
                .sum::<i64>()
        })
        .sum();
    dbg!(part1);
    assert_eq!(part1, 23054);

    let valid_tickets: Vec<&Vec<i64>> = input
        .nearby_tickets
        .iter()
        .filter(|&ticket| {
            ticket
                .iter()
                .all(|field| can_be_valid_value_for_field(*field, &input.fields))
        })
        .collect();

    let mut unknown_fields = input.fields.iter().collect::<HashSet<_>>();
    let mut known_fields = HashMap::new();

    while unknown_fields.len() > 0 {
        let mut known = None;
        let mut index = None;

        for &field in &unknown_fields {
            let indices = (0..input.fields.len())
                .filter(|i| !known_fields.contains_key(i))
                .filter(|&i| {
                    valid_tickets.iter().all(|&ticket| {
                        can_be_valid_value_for_field(ticket[i], std::iter::once(field))
                    })
                })
                .collect::<Vec<_>>();
            if indices.len() == 1 {
                known = Some(field);
                index = Some(indices[0]);
                break;
            }
        }

        known_fields.insert(index.unwrap(), known.unwrap());
        unknown_fields.remove(known.unwrap());
    }

    let part2: i64 = known_fields
        .iter()
        .filter_map(|(&i, &f)| {
            if f.name.starts_with("departure") {
                Some(input.my_ticket[i])
            } else {
                None
            }
        })
        .product();
    dbg!(part2);
    assert_eq!(part2, 51240700105297);
}

fn can_be_valid_value_for_field<'a, F>(value: i64, fields: F) -> bool
where
    F: IntoIterator<Item = &'a Field>,
{
    fields.into_iter().any(|possible_field| {
        possible_field
            .ranges
            .iter()
            .any(|possible_values| possible_values.contains(&value))
    })
}

type FieldRange = Vec<RangeInclusive<i64>>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Field {
    name: String,
    ranges: FieldRange,
}

#[derive(Debug)]
struct TicketData {
    fields: Vec<Field>,
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

            let mut halves = line.split(": ");
            let name = halves.next().expect("field name not given");

            for range in halves
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

            fields.push(Field {
                name: name.to_owned(),
                ranges: this_fields,
            })
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

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_16.txt");
    }
}
