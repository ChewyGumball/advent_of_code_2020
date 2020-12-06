use std::env;
use std::fs::File;
use std::io::{self, BufRead, Lines};
use std::path::Path;

fn parse_group(lines: &mut std::iter::Peekable<Lines<io::BufReader<File>>>) -> usize {
    
    let group_lines: Vec<String> = lines.by_ref().take_while(|line| !line.as_ref().unwrap().is_empty()).collect::<Result<_,_>>().unwrap();
    let questions_answered = group_lines.join("");

    let mut answers = std::collections::HashMap::new();
    for answer in questions_answered.as_str().chars() {
        let current_count = answers.entry(answer).or_insert(0);
       *current_count += 1;
    }

    let member_count = group_lines.len();
    let questions_all_answered = answers.values().filter(|value| **value == member_count).count();

    return questions_all_answered;
}

fn parse_file(file_name: &Path) -> Vec<usize> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file,
    };

    let mut answer_counts: Vec<usize> = Vec::new();

    let mut lines = io::BufReader::new(file).lines().peekable();
    while lines.peek().is_some() {
        answer_counts.push(parse_group(&mut lines));
    }

    return answer_counts;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let answer_counts = parse_file(&input_file);
    let answer_sum: usize = answer_counts.iter().sum();
    print!("Answer Sum: {}", answer_sum);
}
