use std::fs;
use std::collections::HashSet;

fn part1(s: String) -> u64 {
    let v = s.chars().collect::<Vec<char>>();
    let s = &v[..];
    let w = s.windows(4);
    for (i, group) in w.enumerate() {
        let mut set = HashSet::new();
        let mut match_found = true;
        for g in group {
            if set.contains(g) {
                match_found = false;
                break;
            }
            set.insert(g);
        } 
        if match_found {
            return (i +  4) as u64;
        }
    }
    unreachable!();
}

fn part2(s: String) -> u64 {
    let v = s.chars().collect::<Vec<char>>();
    let s = &v[..];
    let w = s.windows(14);
    for (i, group) in w.enumerate() {
        let mut set = HashSet::new();
        let mut match_found = true;
        for g in group {
            if set.contains(g) {
                match_found = false;
                break;
            }
            set.insert(g);
        } 
        if match_found {
            return (i +  14) as u64;
        }
    }
    unreachable!();
}


fn main() -> () {
    let s = fs::read_to_string("in.txt").expect("failed to read file");
    let t = s.clone();
    println!("{}", part1(s));
    println!("{}", part2(t));
}