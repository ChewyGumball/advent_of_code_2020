use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Rule {
    field_name: String,
    valid_ranges: Vec<std::ops::Range<i64>>,
}

#[derive(Debug)]
struct Ticket {
    values: Vec<i64>,
}

fn parse_range(s: &str) -> std::ops::Range<i64> {
    let split: Vec<&str> = s.trim().split("-").collect();

    return std::ops::Range {
        start: split[0].parse::<i64>().unwrap(),
        end: split[1].parse::<i64>().unwrap() + 1,
    };
}

fn parse_rule(line: &String) -> Rule {
    let first_split: Vec<&str> = line.split(":").collect();
    let second_split: Vec<&str> = first_split[1].split("or").collect();

    return Rule {
        field_name: String::from(first_split[0]),
        valid_ranges: second_split.iter().map(|r| parse_range(r)).collect(),
    };
}

fn parse_ticket(line: &String) -> Ticket {
    return Ticket {
        values: line.split(",").map(|n| n.parse::<i64>().unwrap()).collect(),
    };
}

fn parse_file(file_name: &Path) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file,
    };

    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|s| s.unwrap())
        .collect();
    let mut current_line_index = 0;

    let mut rules = Vec::new();
    while lines[current_line_index] != "" {
        rules.push(parse_rule(&lines[current_line_index]));
        current_line_index += 1;
    }

    current_line_index += 2;
    let your_ticket = parse_ticket(&lines[current_line_index]);

    current_line_index += 3;
    let mut tickets = Vec::new();
    while current_line_index < lines.len() {
        tickets.push(parse_ticket(&lines[current_line_index]));
        current_line_index += 1;
    }

    return (rules, your_ticket, tickets);
}

fn is_valid_value(value: &i64, rule: &Rule) -> bool {
    return rule.valid_ranges.iter().any(|range| range.contains(value));
}

fn is_valid_ticket(ticket: &Ticket, rules: &Vec<Rule>) -> bool {
    for value in &ticket.values {
        if !rules.iter().any(|rule| is_valid_value(value, rule)) {
            return false;
        }
    }

    return true;
}

fn extract_sole_matches(matches: &mut HashMap<String, HashSet<usize>>) -> HashMap<String, usize> {
    let mut sole_matches = HashMap::new();
    for (key, value) in matches.iter() {
        if value.len() == 1 {
            sole_matches.insert(key.clone(), *value.iter().next().unwrap());
        }
    }

    for (key, value) in sole_matches.iter() {
        matches.remove(key);
        for v in matches.values_mut() {
            v.remove(value);
        }
    }

    return sole_matches;
}

fn find_field_indices(tickets: &Vec<&Ticket>, rules: &Vec<Rule>) -> HashMap<String, usize> {
    let mut mapping = HashMap::new();
    for i in 0..rules.len() {
        for j in 0..rules.len() {
            if tickets
                .iter()
                .all(|ticket| is_valid_value(&ticket.values[i], &rules[j]))
            {
                let matching_set = mapping.entry(rules[j].field_name.clone()).or_insert(HashSet::new());
                matching_set.insert(i);
            }
        }
    }

    let mut complete_mapping = HashMap::new();
    while !mapping.is_empty() {
        let sole_matches = extract_sole_matches(&mut mapping);

        for (key, value) in sole_matches.iter() {
            complete_mapping.insert(key.clone(), *value);
        }
    }

    return complete_mapping;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let (rules, your_ticket, tickets) = parse_file(&input_file);

    let valid_tickets: Vec<&Ticket> = tickets
        .iter()
        .filter(|ticket| is_valid_ticket(ticket, &rules))
        .collect();
    let field_indices = find_field_indices(&valid_tickets, &rules);

    let mut n = 1;
    for (key, value) in field_indices.iter() {
        println!("Key: {}", key);
        if key.starts_with("departure") {
            n *= your_ticket.values[*value];
        }
    }

    println!("{:?}", n);
}
