use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn parse_file(file_name: &Path) -> (VecDeque<i64>, VecDeque<i64>) {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let lines: Vec<String> = io::BufReader::new(file).lines().map(|l| String::from(l.unwrap().trim())).collect();

    let mut player1_deck = VecDeque::new();

    let mut player2_start_index = 0;
    for i in 1..lines.len() {
        let line = &lines[i];
        if line == "" {
            player2_start_index = i + 2;
            break;
        }

        player1_deck.push_back(line.parse::<i64>().unwrap());
    }
    
    let mut player2_deck = VecDeque::new();
    for i in player2_start_index..lines.len() {
        let line = &lines[i];
        player2_deck.push_back(line.parse::<i64>().unwrap());
    }

    return (player1_deck, player2_deck);
}

fn play(deck1: &mut VecDeque<i64>,deck2: &mut VecDeque<i64>) -> i64 {
    while !(deck1.is_empty() || deck2.is_empty()) {
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();

        assert!(top1 != top2);

        if top1 > top2 {
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
    }

    if deck1.is_empty() {
        return score(deck2);
    } else {
        return score(deck1);
    }
}

fn score(deck: &VecDeque<i64>) -> i64 {
    let mut multiplier = deck.len() as i64;
    let mut s = 0;
    for i in 0..deck.len() {
        s += *deck.get(i).unwrap() * multiplier;
        multiplier -= 1;
    }

    return s;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let (mut player1_deck, mut player2_deck) = parse_file(&input_file);
    println!("Player 1: {:?}", player1_deck);
    println!("Player 2: {:?}", player2_deck);

    let final_score = play(&mut player1_deck, &mut  player2_deck);
    println!("Score: {}", final_score);
}