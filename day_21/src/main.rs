use prelude::*;

fn main() {
    do_main("inputs/day_21.txt");
}

fn do_main(filename: &str) {
    let foods: Vec<Food> = read_lines_from_file(filename)
        .map(|line| line.as_str().into())
        .collect();
    dbg!(foods);
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl From<&str> for Food {
    fn from(line: &str) -> Food {
        let mut parts = line.split(" (contains ");
        let left = parts.next().expect("ingredients not specified");

        let right = parts
            .next()
            .expect("allergens missing")
            .split(")")
            .next()
            .unwrap();

        Food {
            ingredients: left.split_whitespace().map(|i| i.to_owned()).collect(),
            allergens: right.split(", ").map(|i| i.to_owned()).collect(),
        }
    }
}
