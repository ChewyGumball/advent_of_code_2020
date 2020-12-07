use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref BAG_NAME_REGEX: Regex = Regex::new(r"(?P<count>\d+)? ?(?P<name>[a-z]+ [a-z]+) bags?").unwrap();
}

struct Bag {
    contained_bags: HashMap<String, usize>
}

fn parse_rule(line: std::io::Result<String>, bags: &mut HashMap<String, Bag>, file_name: String) {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };
    print!("{}\n", value);

    if value.ends_with("no other bags.") {
        let captures = BAG_NAME_REGEX.captures(&value).unwrap();
        let bag_name = captures.name("name").unwrap();

        bags.entry(String::from(bag_name.as_str())).or_insert(Bag {
            contained_bags: HashMap::new()
        });
    } else {
        let mut captures = BAG_NAME_REGEX.captures_iter(&value);
    
        let target_bag_name = String::from(captures.next().unwrap().name("name").unwrap().as_str());
        let target_bag = bags.entry(target_bag_name.clone()).or_insert(Bag {
            contained_bags: HashMap::new()
        });

        print!("{} -> ", target_bag_name);
        for source_bag in captures {
            let source_bag_name = String::from(source_bag.name("name").unwrap().as_str());
            let count = match source_bag.name("count").unwrap().as_str().parse::<usize>() {
                Ok(num) => num,
                Err(why) => panic!("Couldn't parse the number: {}", why)
            };

            print!("{} {} | ", count, source_bag_name);
            target_bag.contained_bags.insert(source_bag_name, count);
        }
        print!("\n");

        bags.entry(target_bag_name).or_insert(Bag {
            contained_bags: HashMap::new()
        });
    }
}

fn parse_file(file_name: &Path) -> HashMap<String, Bag> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines = io::BufReader::new(file).lines();

    let mut bags = HashMap::new();
    for line in lines {
        parse_rule(line, &mut bags, file_name.display().to_string());
    }

    return bags;
}

fn count_bags(bag_name: &String, count: usize, bags: &HashMap<String, Bag>) -> usize {
    let mut contained_bags: usize = 0;
    let bag = bags.get(bag_name).unwrap();
    for (contained_name, contained_count) in bag.contained_bags.iter() {
        let inner_bags = count_bags(contained_name, *contained_count, bags);
        contained_bags += inner_bags;
        println!("{} contains({})", contained_name, inner_bags);
    }

    return contained_bags * count + count;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let bags = parse_file(&input_file);

    let countained_bags = count_bags(&String::from("shiny gold"), 1, &bags);
    
    print!("Bag Count: {}",countained_bags - 1);
}
