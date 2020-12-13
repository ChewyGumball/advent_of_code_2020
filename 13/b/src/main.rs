use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Bus {
    id: i64,
    offset: i64,
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Vec<Bus> {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    let mut busses = Vec::new();
    let mut current_index = 0;
    for b in value.split(',') {
        if b != "x" {
            busses.push(Bus {
                id: b.parse::<i64>().unwrap(),
                offset: -current_index
            });
        }
        current_index += 1; 
    }

    return busses;
}

fn parse_file(file_name: &Path) -> Vec<Bus> {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut lines = io::BufReader::new(file).lines();
    return parse_line(lines.next().unwrap(), file_name.display().to_string());
}

fn find_mod_inverse(a: i64, n: i64) -> i64 {
    let mut prev_r = n;
    let mut curr_r = a;
    let mut prev_t = 0;
    let mut curr_t = 1;

    while curr_r != 0 {
        let quotient = prev_r / curr_r;

        let new_r = prev_r - quotient * curr_r;
        prev_r = curr_r;
        curr_r = new_r;

        let new_t = prev_t - quotient * curr_t;
        prev_t = curr_t;
        curr_t = new_t;        
    }
    
    if prev_t < 0 {
        prev_t += n;
    }

    return prev_t;
}

fn find_earliest_timestamp(busses: &Vec<Bus>) -> i64 {
    let big_n: i64 = busses.iter().map(|bus| bus.id).product();
    let mut x = 0;

    for bus in busses {
        let y = big_n / bus.id;
        let inverse = find_mod_inverse(y, bus.id);
        println!("Bus {}, offset {} (big_n: {}, y: {}, inverse: {})", bus.id, bus.offset, big_n, y, inverse);

        x += bus.offset * y * inverse;
        x %= big_n;
    }

    return (x + big_n) % big_n;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let busses = parse_file(&input_file);
    let timestamp = find_earliest_timestamp(&busses);

    println!("Number: {}", timestamp);
}