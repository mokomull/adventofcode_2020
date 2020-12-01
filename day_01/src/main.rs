use std::io::BufRead;

fn main() {
    do_main("inputs/day_01.txt");
}

fn do_main(filename: &str) {
    let file = std::fs::File::open(filename).expect("could not open the input");
    let mut expenses = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            if let Ok(expense) = line.as_str().trim().parse::<isize>() {
                expenses.push(expense);
            }
        }
    }

    let mut part1 = None;
    let mut part2 = None;

    'out: for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i != j && expenses[i] + expenses[j] == 2020 {
                println!("Answer is {}", expenses[i] * expenses[j]);
                part1 = Some(expenses[i] * expenses[j]);
                break 'out;
            }
        }
    }

    'out2: for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i == j {
                continue;
            }
            for k in 0..expenses.len() {
                if j == k || i == k {
                    continue;
                }
                if expenses[i] + expenses[j] + expenses[k] == 2020 {
                    println!("Answer is {}", expenses[i] * expenses[j] * expenses[k]);
                    part2 = Some(expenses[i] * expenses[j] * expenses[k]);
                    break 'out2;
                }
            }
        }
    }

    assert_eq!(part1, Some(1019904));
    assert_eq!(part2, Some(176647680));
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_01.txt");
    }
}
