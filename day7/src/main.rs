use std::fs::File;
use std::io::{ BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: u32,
    dirs: Vec<Rc<RefCell<Directory>>>,
    parent: Option<Rc<RefCell<Directory>>>
}

#[derive(Debug, Clone)]
struct Tree {
    root: Rc<RefCell<Directory>>,
    current: Rc<RefCell<Directory>>
}

impl Tree {
    fn new() -> Tree {
        let dir = Directory {
            name: "/".to_string(),
            size: 0,
            dirs: Vec::<Rc::new<Directory>>::new(),
            parent: None
        };
        let cell = Rc::new(RefCell::new(dir));
        Tree {
            root: Rc::clone(&cell),
            current: Rc::clone(&cell) 
        }
    }
    
    fn add_dir(&self, mut dir: Directory) {
        let p = &mut self.current.borrow_mut();
        let new_dir = Rc::new(RefCell::new(dir));
        p.dirs.push(Rc::clone(&new_dir));
    //    new_dir.parent.borrow_mut() = Rc::clone(&self.current);
    }

    /*
    fn change_dir(&mut self, name: &str) {
        match name {
            "/" => self.current = self.root.clone(),// Go to root,
            ".." => {
                match &self.parent {
                    None => println!("Can't go back already root directory"),
                    Some(parent) => *self = parent.clone(),
                }
            }, 
            _ => {
                let mut found: Option<Directory> = None;
                for dir in &self.dirs {
                    if dir.name == name {
                        found = Some(dir.clone());
                    }
                }
                match found {
                    None => println!("No directory found with: {name}"),
                    Some(dir) => *self = dir,
                }
            }
        }
    }*/
}

fn main() {
    let input = File::open("./inputs/day7.txt").unwrap();
    let lines = BufReader::new(input).lines();    
    
    let mut tree = Tree::new();
    tree.add_dir(
        Directory {
            name: "noah".to_string(),
            size: 0,
            dirs: Vec::<Directory>::new(),
            parent: None 
        }
    );
    println!("{:?}",tree); 

    /*
    let mut curr_dir = &mut tree;

    for line in lines {
        let l = line.unwrap();
        let split: Vec<&str> = l.split_whitespace().collect();

        if split[0] == "$" {
            if split[1] == "cd" {
                let dir = split[2];
                change_dir(curr_dir, dir);
            }
        } else {
            if split[0] != "dir" {
                let file_size: u32 = split[0].parse().unwrap();
                curr_dir.size += file_size;
            } 
        }
    }

    let mut total_size: u32 = 0;
    for dir in &dir_count {
        let mut size: u32 = dir.1.size;
        let mut inner_dir: Vec<String> = dir.1.dirs.clone();
        size += recursion(&dir_count, &mut inner_dir, &mut 0);
        if size < 100000 {
        println!("{size}");
            total_size += size;
        }
    }
    println!("Sum of total size of directories with at most 100000 size: {total_size}"); 
    */
}

/*
fn change_dir(current_dir: &mut Directory, name: &str) {
    match name {
        ".." => match current_dir.parent {
            None => println!("Already Root directory"),
            Some(parent) => *current_dir = parent.clone() ,
        },
        "/" => findRoot(current_dir), // Root directory
        _ => {
            let mut parent: Option<&Directory> = None;  
            let mut new: Option<Directory> = None;  
            for d in &current_dir.dirs {
                if d.name == name {
                    parent = Some(current_dir));
                    new = Some(d.clone()); 
                }
            }
            match parent {
                None => println!("lol"),
                Some(p) => {
                    current_dir.parent = Some(p);
                    *current_dir = new.unwrap();
                }
            }
                    /*
                    curr_dir.dirs.push(
                        Directory {
                            name: dir.to_string(),
                            size: 0,
                            dirs: Vec::<Directory>::new(),
                        }
                    );
                    let length = curr_dir.dirs.len() - 1;
                    curr_dir = &mut curr_dir.dirs[length]; 
                    */
        }
    }
}

fn findRoot(current_dir: &mut Directory) {
    while !current_dir.parent.is_none() {
        *current_dir = current_dir.parent.unwrap().clone();
    }
}

fn recursion(dir_count: &HashMap<String,DirData>, d: &mut Vec<String>, size: &mut u32) -> u32 {
    if d.len() == 0 {
        return *size;
    }
    if dir_count.contains_key(&d[0]) {
        let name = d[0].to_string();
       *size += dir_count[&name].size;
        for i in dir_count[&name].dirs.clone() {
            /*if !d.contains(&i) {
                d.push(i.to_string());
            }*/
            d.push(i.to_string());
        }
    }
    d.remove(0);
    recursion(dir_count, d, size)
}*/
