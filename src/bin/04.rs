use std::fs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn part1(s: String) -> u64 {
    let mut score: u64 = 0;
    let elf_pairs = s.split("\n");
    for elf_pair in elf_pairs {
        let mut x = elf_pair.split(",");
        let first = x.next().expect("should have a first value");
        let second = x.next().expect("should have a second value");
        let mut first_split = first.split("-");
        let mut second_split = second.split("-");
        let first_start: u64 = first_split.next().unwrap().parse::<u64>().unwrap();
        let first_end: u64 = first_split.next().unwrap().parse::<u64>().unwrap();
        let second_start: u64 = second_split.next().unwrap().parse::<u64>().unwrap();
        let second_end: u64 = second_split.next().unwrap().parse::<u64>().unwrap();
        let a = first_start..=first_end;
        let b = second_start..=second_end;
        let c = a.clone();
        let d = b.clone();
        let mut b_contains_a = true;
        for x in a {
            if !b.contains(&x) {
                b_contains_a = false;
                break;
            }
        }
        
        let mut c_contains_d = true;
        if !b_contains_a {
            for x in d {
                if !c.contains(&x) {
                    c_contains_d = false;
                    break;
                }
            }
        }

        if b_contains_a || c_contains_d {
            score += 1;
        }
    }
    return score;
}

fn part2(s: String) -> u64 {
    let mut score: u64 = 0;
    let elf_pairs = s.split("\n");
    for elf_pair in elf_pairs {
        let mut x = elf_pair.split(",");
        let first = x.next().expect("should have a first value");
        let second = x.next().expect("should have a second value");
        let mut first_split = first.split("-");
        let mut second_split = second.split("-");
        let first_start: u64 = first_split.next().unwrap().parse::<u64>().unwrap();
        let first_end: u64 = first_split.next().unwrap().parse::<u64>().unwrap();
        let second_start: u64 = second_split.next().unwrap().parse::<u64>().unwrap();
        let second_end: u64 = second_split.next().unwrap().parse::<u64>().unwrap();
        let a = first_start..=first_end;
        let b = second_start..=second_end;
        let c = a.clone();
        let d = b.clone();
        let mut b_contains_a = false;
        for x in a {
            if b.contains(&x) {
                b_contains_a = true;
                break;
            }
        }
        
        let mut c_contains_d = false;
        if !b_contains_a {
            for x in d {
                if c.contains(&x) {
                    c_contains_d = true;
                    break;
                }
            }
        }

        if b_contains_a || c_contains_d {
            score += 1;
        }
    }
    return score;
}

fn main() -> () {
    let s = String::from(include_str!("../inputs/04_in.txt"));
    let t = s.clone();
    println!("{}", part1(s));
    println!("{}", part2(t));
}