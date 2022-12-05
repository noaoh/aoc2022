use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;

fn is_string_numeric(s: &str) -> bool {
    for c in s.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn part1(s: String) -> String {
    let mut item_positions = HashMap::new();
    let mut stack_of_crates: Vec<VecDeque<char>> = Vec::new(); 
    let mut commands: Vec<Vec<u64>> = Vec::new();
    let mut command_parsing_mode = false;
    for (i, line) in s.split("\n").enumerate() {
        if i == 0 {
            let mut pos = 0;
            for (i, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    item_positions.insert(i, pos);
                    pos += 1;
                }
            }
            let l = item_positions.len();
            for _ in 0..l {
                let c: VecDeque<char> = VecDeque::new();
                stack_of_crates.push(c);
            }
        } else if line.is_empty() {
            command_parsing_mode = true;
        } else if !command_parsing_mode { 
            for (i, char) in line.chars().enumerate() {
                if item_positions.contains_key(&i) && char != ' ' {
                    let j = item_positions[&i];
                    stack_of_crates[j].push_back(char);
                }
            }
        } else if command_parsing_mode {
            let mut command = Vec::new();
            for s in line.split(" ") {
                if is_string_numeric(s) {
                    command.push(s.parse::<u64>().unwrap());
                }
            }
            commands.push(command);
        } else {
            unreachable!();
        }
    }

    for command in commands {
        let amount = command[0];
        let from = (command[1] - 1) as usize;
        let to = (command[2] - 1) as usize;
        let mut a = 0; 
        while a != amount {
            let c = stack_of_crates[from].pop_front().unwrap();
            stack_of_crates[to].push_front(c);
            a += 1;
        }
    } 

    let mut s: String = String::from("");
    for c in stack_of_crates {
        s.push(*c.front().unwrap());
    }
    return s;
}

fn part2(s: String) -> String {
    let mut item_positions = HashMap::new();
    let mut stack_of_crates: Vec<VecDeque<char>> = Vec::new(); 
    let mut commands: Vec<Vec<u64>> = Vec::new();
    let mut command_parsing_mode = false;
    for (i, line) in s.split("\n").enumerate() {
        if i == 0 {
            let mut pos = 0;
            for (i, char) in line.chars().enumerate() {
                if char.is_numeric() {
                    item_positions.insert(i, pos);
                    pos += 1;
                }
            }
            let l = item_positions.len();
            for _ in 0..l {
                let c: VecDeque<char> = VecDeque::new();
                stack_of_crates.push(c);
            }
        } else if line.is_empty() {
            command_parsing_mode = true;
        } else if !command_parsing_mode { 
            for (i, char) in line.chars().enumerate() {
                if item_positions.contains_key(&i) && char != ' ' {
                    let j = item_positions[&i];
                    stack_of_crates[j].push_back(char);
                }
            }
        } else if command_parsing_mode {
            let mut command = Vec::new();
            for s in line.split(" ") {
                if is_string_numeric(s) {
                    command.push(s.parse::<u64>().unwrap());
                }
            }
            commands.push(command);
        } else {
            unreachable!();
        }
    }

    for command in commands {
        let amount = command[0];
        let from = (command[1] - 1) as usize;
        let to = (command[2] - 1) as usize;
        let mut a = 0; 
        let mut v = VecDeque::new();
        while a != amount {
            v.push_front(stack_of_crates[from].pop_front().unwrap());
            a += 1;
        }

        a = 0;
        while a != amount {
            stack_of_crates[to].push_front(v.pop_front().unwrap());
            a += 1;
        }
    } 

    let mut s: String = String::from("");
    for c in stack_of_crates {
        s.push(*c.front().unwrap());
    }
    return s;
}

fn main() -> () {
    let s = fs::read_to_string("in.txt").expect("failed to read file");
    let t = s.clone();
    println!("{}", part1(s));
    println!("{}", part2(t));
}