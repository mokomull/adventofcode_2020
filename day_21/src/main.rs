use prelude::*;

fn main() {
    do_main("inputs/day_21.txt");
}

fn do_main(filename: &str) {
    let foods: Vec<Food> = read_lines_from_file(filename)
        .map(|line| line.as_str().into())
        .collect();

    let mut possible_allergens: HashMap<String, HashSet<String>> = HashMap::new();
    for food in &foods {
        for ingredient in &food.ingredients {
            let allergens = food.allergens.iter().cloned().collect::<HashSet<String>>();
            if let Some(set) = possible_allergens.get_mut(ingredient) {
                *set = set.intersection(&allergens).cloned().collect()
            } else {
                possible_allergens.insert(ingredient.clone(), allergens);
            }
        }
    }
    let part1 = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| possible_allergens[ingredient].is_empty())
        .count();
    dbg!(part1);
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
