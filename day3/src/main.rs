use std::fs::File;
use std::io::{ BufRead, BufReader };

const A: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let input = File::open("./inputs/day3.txt").unwrap();
    let input2 = File::open("./inputs/day3.txt").unwrap();
    first_star(&input);
    second_star(&input2);
}

fn first_star(input: &File) {
    let b = BufReader::new(input);
    let mut t_points = 0;
    for line in b.lines() {
        t_points += compute_points(&line.unwrap());
    }
    println!("Total points: {}", t_points);
}

fn second_star(input: &File) {
    let b = BufReader::new(input);
    let mut t_points = 0;
    let mut group: Vec<String> = vec![];
    for line in b.lines() {
        let l = line.unwrap();
        group.push(l.to_string());
        if group.len() == 3 {
            println!("{:?}", group);
            t_points += compute_badge_points(&group);
            group = vec![]; 
        }
    }
    println!("Total points: {}", t_points);
}

// Utils
fn compute_points(item: &String) -> usize {
    let (first, last) = item.split_at(item.len() / 2); 
    for c in first.chars() {
        if last.find(c).is_some() {
            return String::from(A).find(c).unwrap() + 1;
        }
    }
    return 0;
}

fn compute_badge_points(item: &Vec<String>) -> usize {
    for c in item[0].chars() {
        if item[1].find(c).is_some() && item[2].find(c).is_some() {
            let points = String::from(A).find(c).unwrap() + 1;
            println!("{} - {}", c , points);
            return points; 
        }
    }
    return 0;
}
