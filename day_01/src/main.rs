use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();
    let buffered = std::io::BufReader::new(lock);
    let mut expenses = Vec::new();
    for line in buffered.lines() {
        if let Ok(line) = line {
            if let Ok(expense) = line.as_str().trim().parse::<isize>() {
                expenses.push(expense);
            }
        }
    }

    'out: for i in 0..expenses.len() {
        for j in 0..expenses.len() {
            if i != j && expenses[i] + expenses[j] == 2020 {
                println!("Answer is {}", expenses[i] * expenses[j]);
                break 'out;
            }
        }
    }
}
