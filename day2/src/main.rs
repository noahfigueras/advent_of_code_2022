use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    first_star();
    second_star();
}

fn first_star() {
    let input = File::open("./inputs/day2.txt").unwrap();
    let buffer = BufReader::new(input);
    let mut score = 0;

    for line in buffer.lines() {
        let l = line.unwrap();
        score += match Some(&*l) {
             Some("A X") => 4,
             Some("A Y") => 8,
             Some("A Z") => 3,
             Some("B X") => 1,
             Some("B Y") => 5,
             Some("B Z") => 9,
             Some("C X") => 7,
             Some("C Y") => 2,
             Some("C Z") => 6,
             _ => 0
        }
    }

    println!("Your score: {}", score);
}

fn second_star() {
    let input = File::open("./inputs/day2.txt").unwrap();
    let buffer = BufReader::new(input);
    let mut score = 0;

    for line in buffer.lines() {
        let l = line.unwrap();
        score += match Some(&*l) {
             Some("A X") => 3,
             Some("A Y") => 4,
             Some("A Z") => 8,
             Some("B X") => 1,
             Some("B Y") => 5,
             Some("B Z") => 9,
             Some("C X") => 2,
             Some("C Y") => 6,
             Some("C Z") => 7,
             _ => 0
        }
    }

    println!("Your score: {}", score);
}
