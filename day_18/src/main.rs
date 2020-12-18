use prelude::*;

use Token::*;

fn main() {
    do_main("inputs/day_18.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<Token>> = read_lines_from_file(filename)
        .map(|line| parse_line(&line))
        .collect();

    let part1: i64 = input
        .iter()
        .map(|line| evaluate(&line, &HashMap::new()))
        .sum();
    dbg!(part1);
    assert_eq!(part1, 701339185745);

    let part2: i64 = input
        .iter()
        .map(|line| evaluate(&line, &vec![(Asterisk, 1), (Plus, 2)].drain(..).collect()))
        .sum();
    dbg!(part2);
    assert_eq!(part2, 4208490449905);
}

fn evaluate(tokens: &[Token], precedence: &HashMap<Token, i32>) -> i64 {
    let mut rpn_queue = Vec::new();
    let mut op_stack = Vec::new();

    for token in tokens {
        match token {
            t @ Integer(_) => rpn_queue.push(t),
            t @ Plus | t @ Asterisk => {
                while !op_stack.is_empty()
                    // Wikipedia says (the operator at the top of the operator stack has greater
                    // precedence) or (the operator at the top of the operator stack has equal
                    // precedence and the token is left associative), but ALL of our operators are
                    // left-associative.  So just summarize into >=.
                    && precedence.get(*op_stack.last().unwrap()).unwrap_or(&0)
                        >= precedence.get(t).unwrap_or(&0)
                    && op_stack.last().unwrap() != &&LeftParen
                {
                    rpn_queue.push(op_stack.pop().unwrap());
                }
                op_stack.push(t);
            }
            LeftParen => op_stack.push(&LeftParen),
            RightParen => {
                while op_stack.last().unwrap() != &&LeftParen {
                    rpn_queue.push(op_stack.pop().unwrap());
                }
                assert_eq!(op_stack.pop(), Some(&LeftParen));
            }
        }
    }
    while !op_stack.is_empty() {
        let x = op_stack.pop().unwrap();
        assert_ne!(x, &LeftParen);
        rpn_queue.push(x);
    }

    let mut stack: Vec<i64> = Vec::new();

    for token in rpn_queue {
        match token {
            Integer(x) => stack.push(*x),
            Plus => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left + right);
            }
            Asterisk => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();
                stack.push(left * right);
            }
            LeftParen | RightParen => panic!("we should have killed all the parens by now"),
        }
    }

    if let Some(x) = stack.last() {
        *x
    } else {
        panic!("Unexpected stack: {:?}", stack);
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum Token {
    Integer(i64),
    Plus,
    Asterisk,
    LeftParen,
    RightParen,
}

fn parse_line(input: &str) -> Vec<Token> {
    let re = Regex::new("[0-9]+|\\*|\\+|\\(|\\)").unwrap();

    re.find_iter(input)
        .map(|m| match m.as_str() {
            "+" => Plus,
            "*" => Asterisk,
            "(" => LeftParen,
            ")" => RightParen,
            x => Integer(x.parse().expect("not an integer")),
        })
        .collect()
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        super::do_main("../inputs/day_18.txt");
    }
}
