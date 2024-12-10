use crate::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let mut all_allergens: HashMap<_, Vec<_>> = HashMap::new();
    let mut all_ingredients = HashMap::new();
    for line in input.trim().lines() {
        let mut line = line.trim().trim_end_matches(')').split(" (contains ");

        let ingredients = line.next().unwrap();
        let allergens = line.next().unwrap();

        let ingredients: HashSet<_> = ingredients.split_whitespace().collect();

        let allergens = allergens.split(',').map(|a| a.trim());
        for allergen in allergens {
            all_allergens
                .entry(allergen)
                .and_modify(|potential_ingredients| {
                    let mut union: Vec<&str> = Vec::new();
                    for ingredient in potential_ingredients.iter_mut() {
                        if ingredients.contains(*ingredient) {
                            union.push(*ingredient);
                        }
                    }

                    *potential_ingredients = union;
                })
                .or_insert_with(|| ingredients.iter().copied().collect());
        }

        for ingredient in ingredients {
            all_ingredients
                .entry(ingredient)
                .and_modify(|existing_count| *existing_count += 1)
                .or_insert(1);
        }
    }

    for (_allergen, potential_ingredients) in all_allergens {
        for ingredient in potential_ingredients {
            all_ingredients.remove(ingredient);
        }
    }

    all_ingredients.values().sum()
}

pub fn part_two(input: &str) -> String {
    let mut all_allergens: HashMap<_, Vec<_>> = HashMap::new();

    for line in input.trim().lines() {
        let mut line = line.trim().trim_end_matches(')').split(" (contains ");

        let ingredients = line.next().unwrap();
        let allergens = line.next().unwrap();

        let ingredients: HashSet<_> = ingredients.split_whitespace().collect();

        let allergens = allergens.split(',').map(|a| a.trim());
        for allergen in allergens {
            all_allergens
                .entry(allergen)
                .and_modify(|potential_ingredients| {
                    let mut union: Vec<&str> = Vec::new();
                    for ingredient in potential_ingredients.iter_mut() {
                        if ingredients.contains(*ingredient) {
                            union.push(*ingredient);
                        }
                    }

                    *potential_ingredients = union;
                })
                .or_insert_with(|| ingredients.iter().copied().collect());
        }
    }

    let mut allergen_mapping: Vec<(&str, &str)> = Vec::new();

    while allergen_mapping.len() < all_allergens.len() {
        for (allergen, potential_ingredients) in all_allergens.iter_mut() {
            if potential_ingredients.is_empty() {
                continue;
            }

            let mut i = 0;
            while i < potential_ingredients.len() {
                if allergen_mapping
                    .iter()
                    .any(|am| am.1 == potential_ingredients[i])
                {
                    potential_ingredients.remove(i);
                } else {
                    i += 1;
                }
            }

            if potential_ingredients.len() == 1 {
                let ingredient = potential_ingredients.remove(0);
                allergen_mapping.push((*allergen, ingredient));
            }
        }
    }

    allergen_mapping.sort_by_key(|am| am.0);

    let mut result = String::new();
    let mut first = true;
    for (_allergen, ingredient) in allergen_mapping {
        if !first {
            result.push(',');
        }
        result.push_str(ingredient);
        first = false;
    }

    result
}

#[test]
fn test() {
    let run_a = |input, res| assert_eq!(part_one(input), res);
    let run_b = |input, res| assert_eq!(part_two(input), res);

    let i = r#"
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"#;

    run_a(i, 5);
    run_b(i, "mxmxvkd,sqjhc,fvjkl");
}
