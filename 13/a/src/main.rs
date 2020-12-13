use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Schedule {
    earliest_departure: i32,
    busses: Vec<i32>,
}

fn parse_line(line: std::io::Result<String>, file_name: String) -> Vec<i32> {
    let value = match line {
        Err(why) => panic!("Couldn't read line from {}: {}", file_name, why),
        Ok(line) => line
    };

    return value.split(',').filter(|id| *id != "x").map(|id| id.parse::<i32>().unwrap()).collect();
}

fn parse_file(file_name: &Path) -> Schedule {
    let file = match File::open(&file_name) {
        Err(why) => panic!("Couldn't open {}: {}", file_name.display(), why),
        Ok(file) => file
    };

    let mut lines = io::BufReader::new(file).lines();
    let earliest_departure = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let busses = parse_line(lines.next().unwrap(), file_name.display().to_string());

    return Schedule {
        earliest_departure: earliest_departure,
        busses: busses
    };
}

fn find_earliest_bus(schedule: &Schedule) -> (i32, i32) {
    let mut earliest_bus_id = -42;
    let mut least_wait_time = std::i32::MAX;

    for bus in &schedule.busses {
        let modulo = schedule.earliest_departure % bus;
        let wait_time = bus - modulo;
        
        if wait_time < least_wait_time {
            earliest_bus_id = *bus;
            least_wait_time = wait_time;
        }
    }

    return (earliest_bus_id, least_wait_time);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);

    let schedule = parse_file(&input_file);
    let (bus_id, wait_time) = find_earliest_bus(&schedule);

    println!("Number: {}", bus_id * wait_time);
}