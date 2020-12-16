use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
    let mut other_tickets = Vec::new();
    while current_line_index < lines.len() {
        other_tickets.push(parse_ticket(&lines[current_line_index]));
        current_line_index += 1;
    }

    return (rules, your_ticket, other_tickets);
}

fn find_invalid_values(ticket: &Ticket, rules: &Vec<Rule>) -> Vec<i64> {
    let mut invalid_values = Vec::new();
    for value in &ticket.values {
        if !rules
            .iter()
            .any(|rule| rule.valid_ranges.iter().any(|range| range.contains(&value)))
        {
            invalid_values.push(*value);
        }
    }

    return invalid_values;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let (rules, your_ticket, nearby_tickets) = parse_file(&input_file);

    let invalid_values: i64 = nearby_tickets
        .iter()
        .map(|ticket| find_invalid_values(ticket, &rules))
        .flatten()
        .sum();

    println!("{:?}", invalid_values);
}
