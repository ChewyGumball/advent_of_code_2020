use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;
use lazy_static::lazy_static;


lazy_static! {
    static ref BAG_NAME_REGEX: Regex = Regex::new(r"(?P<name>[a-z]+ [a-z]+) bags?").unwrap();
}

struct Bag {
    possible_containers: Vec<String>
}

fn parse_rule(line: std::io::Result<String>, bags: &mut HashMap<String, Bag>, file_name: String) {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };
    print!("{}\n", value);

    if value.ends_with("no other bags.") {
        let captures = BAG_NAME_REGEX.captures(&value).unwrap();
        let bag_name = captures.get(1).unwrap();

        bags.entry(String::from(bag_name.as_str())).or_insert(Bag {
            possible_containers: Vec::new()
        });
    } else {
        let mut captures = BAG_NAME_REGEX.captures_iter(&value);
    
        let target_bag_name = String::from(captures.next().unwrap().name("name").unwrap().as_str());
        print!("{} -> ", target_bag_name);
        for source_bag in captures {
            let source_bag_name = String::from(source_bag.name("name").unwrap().as_str());
            print!("{} | ", source_bag_name);
            bags.entry(source_bag_name).or_insert(Bag {
                possible_containers: Vec::new()
            }).possible_containers.push(target_bag_name.clone());
        }
        print!("\n");

        bags.entry(target_bag_name).or_insert(Bag {
            possible_containers: Vec::new()
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let bags = parse_file(&input_file);

    let mut containing_bags = HashSet::new();
    let mut bags_to_check = VecDeque::new();
    bags_to_check.push_back(String::from("shiny gold"));

    while !bags_to_check.is_empty() {
        let next_bag = bags_to_check.pop_front().unwrap();
        match bags.get(&next_bag) {
            Some(value) => {
                for parent_bag in &value.possible_containers {
                    containing_bags.insert(parent_bag.to_string());
                    bags_to_check.push_back(parent_bag.to_string());
                }
            },
            None => panic!("What?")
        };
    }
    
    print!("Bag Count: {}", containing_bags.len());
}
