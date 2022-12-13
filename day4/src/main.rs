use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day4.txt").unwrap();
    let input2 = File::open("./inputs/day4.txt").unwrap();
    first_star(&input);
    second_star(&input2);
}

fn first_star(input: &File) {
    let b = BufReader::new(input);
    let mut counter = 0;
    for line in b.lines() {
        let l = line.unwrap();
        let pair: Vec<&str> = l.split(&[',', '-'][..]).collect();
        let ranges: Vec<i32> = vec![
            pair[0].parse::<i32>().unwrap(),
            pair[1].parse::<i32>().unwrap(),
            pair[2].parse::<i32>().unwrap(),
            pair[3].parse::<i32>().unwrap()
        ];
        if (ranges[2] >= ranges[0] && ranges[3] <= ranges[1]) || 
            (ranges[0] >= ranges[2] && ranges[1] <= ranges[3])
        {
            counter += 1;
        } 
    }
    println!("Total:{}", counter);
}

fn second_star(input: &File) {
    let b = BufReader::new(input);
    let mut counter:u32 = 0;
    for line in b.lines() {
        let l = line.unwrap();
        let pair: Vec<&str> = l.split(&[',', '-'][..]).collect();
        let ranges: Vec<i32> = vec![
            pair[0].parse::<i32>().unwrap(),
            pair[1].parse::<i32>().unwrap(),
            pair[2].parse::<i32>().unwrap(),
            pair[3].parse::<i32>().unwrap()
        ];
        counter += find_overlaps(ranges) as u32;
    }
    println!("Total:{}", counter);
}

fn find_overlaps(pair: Vec<i32>) -> u8 {
    let range: Vec<i32>;
    let iter: Vec<i32>;

    if pair[0] < pair[2] {
        range = pair[0..2].to_vec();
        iter = pair[2..].to_vec();
    } else {
        range = pair[2..].to_vec();
        iter = pair[0..2].to_vec();
    }
    
    for number in iter[0]..iter[1]+1 {
        if (number >= range[0]) && (number <= range[1]) {
            return 1;
        }
    }
    return 0;
}
