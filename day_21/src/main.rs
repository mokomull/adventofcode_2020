use prelude::*;

fn main() {
    do_main("inputs/day_21.txt");
}

fn do_main(filename: &str) {
    let foods: Vec<Food> = read_lines_from_file(filename)
        .map(|line| line.as_str().into())
        .collect();

    let mut possible_allergens: HashMap<String, HashSet<String>> = HashMap::new();
    for food in foods {
        for ingredient in &food.ingredients {
            let set = possible_allergens.entry(ingredient.clone()).or_default();

            for allergen in &food.allergens {
                set.insert(allergen.clone());
            }
        }
    }
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
