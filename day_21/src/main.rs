use prelude::*;

fn main() {
    do_main("inputs/day_21.txt");
}

fn do_main(filename: &str) {
    let foods: Vec<Food> = read_lines_from_file(filename)
        .map(|line| line.as_str().into())
        .collect();

    let mut allergen_to_ingredient: HashMap<String, HashSet<String>> = HashMap::new();
    for food in &foods {
        for allergen in &food.allergens {
            let ingredients = food
                .ingredients
                .iter()
                .cloned()
                .collect::<HashSet<String>>();
            if let Some(set) = allergen_to_ingredient.get_mut(allergen) {
                *set = set.intersection(&ingredients).cloned().collect()
            } else {
                allergen_to_ingredient.insert(allergen.clone(), ingredients);
            }
        }
    }
    dbg!(&allergen_to_ingredient);
    let part1 = foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| {
            allergen_to_ingredient
                .values()
                .all(|ingredients| !ingredients.contains(ingredient))
        })
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
