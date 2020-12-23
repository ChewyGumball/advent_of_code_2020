use std::collections::VecDeque;

fn rotate_to(target: usize, queue: &mut VecDeque<usize>) {
    while *queue.front().unwrap() != target {
        queue.rotate_left(1);
    }
}

fn play(input: Vec<usize>) -> VecDeque<usize> {
    let mut cups = VecDeque::from(input);

    for i in 0..100 {
        println!("{}", i);
        let current_cup = cups.pop_front().unwrap();
        let mut rest = cups.split_off(3);

        let mut destination_cup = current_cup - 1;
        if destination_cup == 0 {
            destination_cup = 9;
        }

        while cups.contains(&destination_cup) {
            println!("Decrementing");
            destination_cup = destination_cup - 1;
            if destination_cup == 0 {
                destination_cup = 9;
            }    
        }

        rest.push_front(current_cup);
        rotate_to(destination_cup, &mut rest);
        rest.rotate_left(1);

        for cup in cups.iter().rev() {
            rest.push_front(*cup);
        }

        rotate_to(current_cup, &mut rest);
        rest.rotate_left(1);

        cups = rest;
    }

    rotate_to(1, &mut cups);

    return cups;
}

fn main() {
    let input = vec![1,9,8,7,5,3,4,6,2];
    let end = play(input);

    println!("{:?}", end);
}
