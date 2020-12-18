use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn break_expression_line(line: String) -> Vec<String> {
    let expanded = line.replace("(", "( ").replace(")", " )");
    return expanded.split(" ").map(|s| String::from(s)).collect();
} 

fn parse_file(file_name: &Path) -> Vec<Vec<String>> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut lines = io::BufReader::new(file).lines();

    return lines.map(|l| break_expression_line(l.unwrap())).collect();
}

fn parse_number(expression: &[String]) -> (i64, &[String]) {
    return (expression[0].parse::<i64>().unwrap(), &expression[1..]);
}

fn parse_expression(expression: &[String]) -> (i64, &[String]) {
    //println!("Expression: {:?}", expression);
    let (mut left, mut current_expression) = parse_value(expression);
    while !current_expression.is_empty() && current_expression[0] != ")" {
        let (right, next_expr) = parse_value(&current_expression[1..]);
        //println!("Left: {}, Right: {}, Op: {}", left, right, current_expression[0]);
        match current_expression[0].as_str() {
            "*" => left = left * right,
            "+" => left = left + right,
            _ => panic!("Unknown operator: {}", expression[0])
        }
        current_expression = next_expr;
    }

    if !current_expression.is_empty() {
        current_expression = &current_expression[1..];
    }

    return (left, current_expression);
}

fn parse_value(expression: &[String]) -> (i64, &[String]) {
   // println!("{:?}", expression);
    return match expression[0].as_str() {
        "(" => parse_expression(&expression[1..]),
        _ => match expression[0].parse::<i64>() {
            Ok(v) => (v, &expression[1..]),
            Err(e) => panic!("Couldn't parse {} as a value: {}", expression[0], e),
        }, 
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let expressions = parse_file(&input_file);

    let results: Vec<i64> = expressions.iter().map(|e| parse_expression(e).0).collect();
    println!("{:?}", results.iter().sum::<i64>());
}