use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let input = File::open("./inputs/day8.txt").unwrap();
    let lines = BufReader::new(input).lines();    

    // Create Grid
    let mut grid: Vec<Vec<u8>> = vec![]; 
    for line in lines {
        let l = line.unwrap();
        let mut row: Vec<u8> = vec![];
        for number in l.chars() {
            let num: u8 = number.to_string().parse().unwrap();
            row.push(num);
        }
        grid.push(row);
    }

    // Find Visible trees
    let mut count = 0;
    let grid_len = grid.len();
    for (i, row) in grid.iter().enumerate() {
        let len = row.len();
        for (x, _tree) in row.iter().enumerate() {
            if x == 0 || x == len -1 {
                count += 1; // Edge
            } else if i == 0 || i == grid_len - 1 {
                count += 1; // Edge
            } else if check_visibility(&grid, x, i) {
                count += 1;
            }
        }
    }

    // Find highest scenic score 
    let mut highest_score = 0;
    for (i, row) in grid.iter().enumerate() {
        let len = row.len();
        for (x, _tree) in row.iter().enumerate() {
            if x == 0 || x == len -1 {
                continue; // Edge
            } else if i == 0 || i == grid_len - 1 {
                continue; // Edge
            } else { 
                let score = check_score(&grid, x, i);
                if score > highest_score {
                    highest_score = score;
                }
            }
        }
    }

    println!("Visible trees: {count}");
    println!("Highest scenic score: {highest_score}");
}

fn check_score(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> u32 {
    let mut score_left = 0;
    let mut score_right = 0;
    let mut score_top = 0;
    let mut score_down = 0;

    let element = grid[y][x];

    // Check left 
    for tree in grid[y][..x].iter().rev() {
        score_left += 1;
        if tree >= &element {
            break;
        }
    }
    
    // Check right 
    for tree in grid[y][x+1..].iter() {
        score_right += 1;
        if tree >= &element {
            break;
        }
    }
    
    // Check top 
    for row in grid[..y].iter().rev() {
        score_top += 1;
        if row[x] >= element {
            break;
        }
    }
    
    // Check down 
    for row in grid[y+1..].iter() {
        score_down += 1;
        if row[x] >= element {
            break;
        }
    }

    //println!("{element}: left: {score_left}, right: {score_right}");
    return score_left * score_right * score_top * score_down;
}

fn check_visibility(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    let mut visible_left = true;
    let mut visible_right = true;
    let mut visible_top = true;
    let mut visible_down = true;

    let element = grid[y][x];

    // Check x 
    for (i, tree) in grid[y].iter().enumerate() {
        if i == x { 
            continue; 
        } else if i < x && tree >= &element {
            visible_left = false;
        } else if i > x && tree >= &element {
            visible_right = false;
        }
    }
   
    // Check y 
    for(i, row) in grid.iter().enumerate() {
       if i == y {
            continue;
        } else if i < y && row[x] >= element {
            visible_top = false;
        } else if i > y && row[x] >= element {
            visible_down = false;
        }
    }

    return visible_left || visible_right || visible_top || visible_down;
}
