use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day5.txt").unwrap();
    let input2 = File::open("./inputs/day5.txt").unwrap();
    first_star(&input);
    second_star(&input2);
}

fn first_star(input: &File) {
    let b = BufReader::new(input);    
    let mut crates: Vec<String> = get_crates();
    for line in b.lines() {
        let l = line.unwrap();
        let changes: Vec<u32> = get_changes(l);
        for _crate in 0..changes[0] {
            let from: usize = (changes[1]-1).try_into().unwrap();
            let to: usize = (changes[2]-1).try_into().unwrap();
            let p = crates[from].pop().unwrap();
            crates[to].push(p);
        }
    }
    println!("{:?}", crates);
}

fn second_star(input: &File) {
    let b = BufReader::new(input);    
    let mut crates: Vec<String> = get_crates();
    for line in b.lines() {
        let l = line.unwrap();
        let changes: Vec<u32> = get_changes(l);
        let from: usize = (changes[1]-1).try_into().unwrap();
        let to: usize = (changes[2]-1).try_into().unwrap();
        let moves: usize = (changes[0]).try_into().unwrap();
        let p = crates[from][crates[from].len()-moves..].to_string();
        crates[to] += &p;
        crates[from] = crates[from][0..crates[from].len()-moves].to_string();
    }
    println!("{:?}", crates);
}

fn get_crates() -> Vec<String> {
    return vec![
        "SZPDLBFC".to_string(),
        "NVGPHWB".to_string(),
        "FWBJG".to_string(),
        "GJNFLWCS".to_string(),
        "WJLTPMSH".to_string(),
        "BCWGFS".to_string(),
        "HTPMQBW".to_string(),
        "FSWT".to_string(),
        "RCN".to_string()
    ];
}

fn get_changes(s: String) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut i = 0;
    let mut flag: bool = false;
    for c in s.chars() {
        let n = c as u32;
        if n >= 48 && n <= 57 {
            let b = s.chars().nth(i+1);
            if b.is_some() && (b != Some(' ')) {
                let mut concat = String::from(c);
                concat.push(b.unwrap());
                v.push(concat.parse::<u32>().unwrap());
                flag = true;
            } else if flag {
                flag = false;
            } else {
                let a = c.to_string().parse::<u32>().unwrap();
                v.push(a);
            }
        }
        i += 1;
    }
    return v;
}
