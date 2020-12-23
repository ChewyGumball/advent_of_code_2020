use std::time::Instant;

fn print_turn(cups: &Vec<usize>, current_cup: usize, destination_cup: usize) {
    print!("cups: ");
    let mut current = 1;
    for _ in 1..cups.len() {
        if current == current_cup {
            print!("({})", current);
        } else {
            print!(" {} ", current);
        }

        current = cups[current];
    }
    println!("");
    println!("pick up: {}, {}, {}", cups[current_cup],cups[cups[current_cup]],cups[cups[cups[current_cup]]]);
    println!("destination: {}\n", destination_cup);
}

fn play(input: Vec<usize>) -> Vec<usize> {
    let mut next = vec![0;input.len() + 1];
    let last = input.len() - 1;
    //println!("{:?}", input);
    for i in 0..last {
        //println!("{} -> {}", input[i], input[i+1]);
        next[input[i]] = input[i + 1];
    }
    next[input[last]] = input[0];

    let mut current_cup = input[last];
    let start = Instant::now();
    let mut iteration_time = Instant::now();

    for i in 0..10000000 {
        if i % 10000 == 0 {
            let elapsed = start.elapsed().as_secs_f64();
            let iteration_elapsed = iteration_time.elapsed().as_secs_f64();
            println!("{}: {}s ({}/s)", i, elapsed, 1000 as f64/iteration_elapsed);
            
            iteration_time = Instant::now();
        }
        current_cup = next[current_cup];

        let next1 = next[current_cup];
        let next2 = next[next1];
        let next3 = next[next2];

        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = input.len();
        }

        while destination_cup == next1 || destination_cup == next2 || destination_cup == next3 {
            destination_cup = destination_cup - 1;
            if destination_cup == 0 {
                destination_cup = input.len();
            }    
        }

        //print_turn(&next, current_cup, destination_cup);

        next[current_cup] = next[next3];
        next[next3] = next[destination_cup];
        next[destination_cup] = next1;
    }

    return next;
}

fn print_cups(cups: &Vec<usize>, count: usize) {
    let mut current_cup = 1;
    for _ in 0..count {
        print!("{} ", cups[current_cup]);
        current_cup = cups[current_cup];
    }
    println!("");
}

fn main() {
    let mut input = vec![1,9,8,7,5,3,4,6,2];
    for i in 10..=1000000 {
        input.push(i);
    }
    let cups = play(input);
    print_cups(&cups, 8);
}
