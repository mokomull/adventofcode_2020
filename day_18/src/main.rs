use prelude::*;

use Token::*;

fn main() {
    do_main("inputs/day_18.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<Token>> = read_lines_from_file(filename)
        .map(|line| parse_line(&line))
        .collect();

    let part2: i64 = input.iter().map(|line| evaluate(&line)).sum();
    dbg!(part2);
}

fn evaluate(tokens: &[Token]) -> i64 {
    let mut rpn_queue = Vec::new();
    let mut op_stack = Vec::new();

    for token in tokens {
        match token {
            t @ Integer(_) => rpn_queue.push(t),
            Plus => {
                while !op_stack.is_empty()
                    && op_stack.last().unwrap() != &&Asterisk
                    && op_stack.last().unwrap() != &&LeftParen
                {
                    rpn_queue.push(op_stack.pop().unwrap());
                }
                op_stack.push(&Plus);
            }
            Asterisk => {
                while !op_stack.is_empty() && op_stack.last().unwrap() != &&LeftParen {
                    rpn_queue.push(op_stack.pop().unwrap());
                }
                op_stack.push(&Asterisk);
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

#[derive(Debug, Eq, PartialEq)]
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
