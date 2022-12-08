use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day1.txt").unwrap();
    let buffer = BufReader::new(input);

    #[derive(Debug, Copy, Clone)]
    struct Elf {
        number: i32,
        calories: i32
    }

    let mut elfs: Vec<Elf> = vec![Elf{number: 0, calories: 0}, Elf{number: 0, calories: 0}, Elf{number: 0, calories: 0}];
    let mut elf = 1;
    let mut calories = 0;

    for line in buffer.lines() {
        let food = line.unwrap();
        if food != "" {
            let num: i32 = food.parse().unwrap();
            calories += num; 
        } else {
            if (calories > elfs[0].calories) && (calories < elfs[1].calories) {
                elfs[0].number = elf;
                elfs[0].calories = calories;
            } else if (calories > elfs[1].calories) && (calories < elfs[2].calories){
                elfs[0] = elfs[1];
                elfs[1].number = elf;
                elfs[1].calories = calories;
            } else if calories > elfs[2].calories {
                elfs[0] = elfs[1];
                elfs[1] = elfs[2];
                elfs[2].number = elf;
                elfs[2].calories = calories;
            }
            elf += 1;
            calories = 0;
        }
    }

    println!("Elf Leaderboard");
    println!("----------------");
    println!("First: Elf {} with {} calories.", elfs[2].number, elfs[2].calories);
    println!("Second: Elf {} with {} calories.", elfs[1].number, elfs[1].calories);
    println!("Third: Elf {} with {} calories.", elfs[0].number, elfs[0].calories);
    println!("----------------");
    println!("Total calories: {}", elfs[0].calories + elfs[1].calories + elfs[2].calories);
}

