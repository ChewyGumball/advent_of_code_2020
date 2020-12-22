use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;
use std::iter::FromIterator;

fn parse_file(file_name: &Path) -> (VecDeque<usize>, VecDeque<usize>) {
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

        player1_deck.push_back(line.parse::<usize>().unwrap());
    }
    
    let mut player2_deck = VecDeque::new();
    for i in player2_start_index..lines.len() {
        let line = &lines[i];
        player2_deck.push_back(line.parse::<usize>().unwrap());
    }

    return (player1_deck, player2_deck);
}

// returns true if player 1 wins
fn play(deck1: &mut VecDeque<usize>,deck2: &mut VecDeque<usize>) -> bool {
    let mut previous_decks = Vec::new();
    while !(deck1.is_empty() || deck2.is_empty()) {
        for (d1, d2) in previous_decks.iter() {
            if d1 == deck1 && d2 == deck2 {
                return true;
            }
        }

        previous_decks.push((deck1.clone(), deck2.clone()));

        
        let top1 = deck1.pop_front().unwrap();
        let top2 = deck2.pop_front().unwrap();
        
        assert!(top1 != top2);

        let mut player1_wins = top1 > top2;
        if deck1.len() >= top1 && deck2.len() >= top2{
            let mut new_deck1 = VecDeque::from_iter(deck1.iter().take(top1).cloned());
            let mut new_deck2 = VecDeque::from_iter(deck2.iter().take(top2).cloned());

            player1_wins = play(&mut new_deck1, &mut new_deck2);
        }

        if player1_wins {
            deck1.push_back(top1);
            deck1.push_back(top2);
        } else {
            deck2.push_back(top2);
            deck2.push_back(top1);
        }
    }

    return deck2.is_empty();
}

fn score(deck: &VecDeque<usize>) -> usize {
    let mut multiplier = deck.len();
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

    let player1_wins = play(&mut player1_deck, &mut  player2_deck);
    if player1_wins {
        println!("Score: {}", score(&player1_deck));
    } else {
        println!("Score: {}", score(&player2_deck));
    }
}