use prelude::*;

use Token::*;

fn main() {
    do_main("inputs/day_18.txt");
}

fn do_main(filename: &str) {
    let input: Vec<Vec<Token>> = read_lines_from_file(filename)
        .map(|line| parse_line(&line))
        .collect();
}

enum Token {
    Integer(i32),
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
