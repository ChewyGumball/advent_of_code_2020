use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn break_expression_line(line: String) -> Vec<String> {
    let mut full = vec![String::from("(")];
    let expanded = line.replace("(", "( ").replace(")", " )");
    full.append(&mut expanded.split(" ").map(|s| String::from(s)).collect::<Vec<String>>());
    full.push(String::from(")"));

    return full;
} 

fn parse_file(file_name: &Path) -> Vec<Vec<String>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut lines = io::BufReader::new(file).lines();

    return lines.map(|l| break_expression_line(l.unwrap())).collect();
}

fn parse_multiplication(expression: &[String]) -> (i64, &[String]) {
    let (mut left, mut remaining) = parse_addition(expression);
    while !remaining.is_empty() && remaining[0] == "*" {
        let (right, remaining2) = parse_addition(&remaining[1..]);
        //println!("{} * {}", left, right);
        left *= right;
        remaining = remaining2;
    }

    return (left, remaining);
}

fn parse_addition(expression: &[String]) -> (i64, &[String]) {
    let (mut left, mut remaining) = parse_value(expression);

    while !remaining.is_empty() && remaining[0] == "+" {
        let (right, remaining2) = parse_value(&remaining[1..]);
        //println!("{} + {}", left, right);
        left += right;
        remaining = remaining2;
    }

    return (left, remaining);
}

fn parse_expression(expression: &[String]) -> (i64, &[String]) {
    assert!(expression[0] == "(");

    //println!("Expression: {:?}", expression);
    let (value, current_expression) = parse_multiplication(&expression[1..]);

    //println!("expression: {:?}", current_expression);
    assert!(current_expression[0] == ")");

    return (value, &current_expression[1..]);
}

fn parse_value(expression: &[String]) -> (i64, &[String]) {
    let (value, rest) = match expression[0].as_str() {
        "(" => parse_expression(expression),
        _ => match expression[0].parse::<i64>() {
            Ok(v) => (v, &expression[1..]),
            Err(e) => panic!("Couldn't parse {} as a value: {}", expression[0], e),
        }, 
    };

    //println!("Value: {}", value);
    return (value, rest);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let expressions = parse_file(&input_file);

    let results: Vec<i64> = expressions.iter().map(|e| parse_expression(e).0).collect();
    println!("{:?}", results);
    println!("{:?}", results.iter().sum::<i64>());
}