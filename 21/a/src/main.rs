use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

#[derive(Debug)]
struct IngredientList {
    ingredients: Vec<String>,
    allergens: Vec<String>
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> IngredientList {
    let s = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    let contains_location = s.find("(contains").unwrap();

    let (ingredients, allergens) = s.split_at(contains_location);

    let i = ingredients.trim().split(" ");
    let a = allergens[9..(allergens.len() - 1)].trim().split(", ");

    return IngredientList {
        ingredients: i.map(|s| String::from(s)).collect(),
        allergens: a.map(|s| String::from(s)).collect(),
    };
}

fn parse_file(file_name: &Path) -> Vec<IngredientList> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    return lines.map(|line| parse_line(line, file_name.display().to_string()))
                .collect();
}

fn make_allergen_sets(lists: &Vec<IngredientList>) -> HashMap<String, Vec<HashSet<String>>> {
    let mut map = HashMap::new();

    for list in lists {
        for allergen in &list.allergens {
            let entry = map.entry(allergen.clone()).or_insert_with(|| Vec::new());
            entry.push(HashSet::from_iter(list.ingredients.iter().cloned()));
        }
    }

    return map;
}

fn condense_sets(sets: &HashMap<String, Vec<HashSet<String>>>) -> HashMap<String, HashSet<String>> {
    let mut condensed = HashMap::new();
    for (allergen, possibilities) in sets.iter() {
        let mut combined = possibilities[0].clone();
        for possibility in possibilities {
            combined = combined.intersection(possibility).map(|a| a.clone()).collect();
        }

        condensed.insert(allergen.clone(), combined);
    }

    return condensed;
}

fn find_single_values(sets: &HashMap<String, HashSet<String>>) -> Vec<String> {
    let mut singles = Vec::new();
    for set in sets.values() {
        if set.len() == 1 {
            singles.push(set.iter().next().unwrap().clone());
        }
    }

    return singles;
}

fn reduce(sets: &mut HashMap<String, HashSet<String>>) {
    let mut singles = find_single_values(sets);
    println!("Original: {:?}", sets);
    while singles.len() != sets.len() {
        for set in sets.values_mut() {
            if set.len() != 1 {
                for single in &singles {
                    set.remove(single);
                }
            }
        }
        println!("Reduce: {:?}", sets);

        singles = find_single_values(sets);
    }
}

fn get_allergen_ingredients(sets: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut ingredients = HashSet::new();
    for s in sets.values() {
        ingredients.insert(s.iter().next().unwrap().clone());
    }

    return ingredients;
}

fn count_ingredients(allergen_ingredients: &HashSet<String>, ingredient_lists: &Vec<IngredientList>) -> usize {
    let mut count = 0;
    for list in ingredient_lists {
        count += list.ingredients.iter().filter(|i| !allergen_ingredients.contains(*i)).count();
    }

    return count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let ingredient_lists = parse_file(&input_file);
    let allergen_sets = make_allergen_sets(&ingredient_lists);

    let mut condensed = condense_sets(&allergen_sets);
    reduce(&mut condensed);

    let allergen_ingredients = get_allergen_ingredients(&condensed);

    let non_allergen_ingredient_count = count_ingredients(&allergen_ingredients, &ingredient_lists);

    println!("{:?}", non_allergen_ingredient_count);
}