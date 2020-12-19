use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;

enum Rule {
    Matches(Vec<Vec<usize>>),
    Atom(char),
}

//let regex: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<value>.+)$").unwrap();

fn parse_rule_part(part: &str) -> Vec<usize> {
    return part.split(" ").filter(|p| *p != "").map(|p| p.parse::<usize>().unwrap()).collect();
}

fn parse_rule(line: &str) -> Rule {
    if line.starts_with("\"") {
        return Rule::Atom(line.chars().nth(1).unwrap());
    }

    let parts = line.split("|");

    return Rule::Matches(parts.map(|p| parse_rule_part(p)).collect());
}

fn make_regex_helper(current_rule_index: usize, rules: &HashMap<usize, Rule>, result: &mut Vec<char>) {
    match current_rule_index {
        8 => {
            result.push('(');
            make_regex_helper(42, rules, result);
            result.push('+');
            result.push(')');
        },
        11 => {
            result.push('(');
            make_regex_helper(42, rules, result);
            result.append(&mut "{%}".chars().collect());
            make_regex_helper(31, rules, result);
            result.append(&mut "{%}".chars().collect());
            result.push(')');
        },
        _ => {
            let current_rule = rules.get(&current_rule_index).unwrap();

            match current_rule {
                Rule::Atom(s) => result.push(*s),
                Rule::Matches(matches) => {
                    result.push('(');
                    for m in matches {
                        for s in m {
                            make_regex_helper(*s, rules, result);
                        }
                        result.push('|');
                    }
                    assert!(*result.last().unwrap() == '|');
                    result.pop();
                    result.push(')');
                }
            }
        }
    };
}

fn make_regex(rules: &HashMap<usize, Rule>) -> String {
    let mut s = Vec::new();
    s.push('^');
    make_regex_helper(0, rules, &mut s);
    s.push('$');

    let st: String = s.iter().collect();
    println!("Regex: {}", st);
    return st;
}

fn parse_file(file_name: &Path) -> (String, Vec<String>) {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| String::from(l.unwrap().trim())).collect();

    let mut rules = HashMap::new();
    let mut current_line_index = 0;

    while lines[current_line_index] != "" {
        let line = &lines[current_line_index];
        let sep = line.find(':').unwrap();

        let (num, rest) = line.split_at(sep);
        rules.insert(num.parse::<usize>().unwrap(), parse_rule(rest[1..].trim()));
        current_line_index += 1;
    }

    current_line_index += 1;

    return (make_regex(&rules), lines[current_line_index..].to_vec());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let (r, messages) = parse_file(&input_file);

    let mut matches = 0;
    let mut current_messages: Vec<&String> = messages.iter().collect();

    for i in 1..100 {
        let regex = Regex::new(&r.replace("%", &i.to_string())).unwrap();
        let matching = current_messages.iter().filter(|m| regex.is_match(m));

        matches += matching.count();
        current_messages = current_messages.iter().filter(|m| !regex.is_match(m)).map(|m| *m).collect();
        println!("Matching: {} ({})", matches, i);
    }

    print!("Valid messages: {}", matches);
}