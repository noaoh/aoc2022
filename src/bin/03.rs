use std::fs;
use std::collections::HashSet;

fn part1(s: String) -> u64 {
    let mut res: u64 = 0;
    let it = s.split("\n");
    for rucksack in it {
        let l = rucksack.chars().count();
        let half = l / 2;
        let first_comp = rucksack[0..half].chars();
        let second_comp = rucksack[half..l].chars();
        let mut first_h = HashSet::new();
        let mut second_h = HashSet::new(); 

        for f in first_comp {
            first_h.insert(f);
        }

        for s in second_comp {
            second_h.insert(s);
        }

        let h: HashSet<_> = first_h.intersection(&second_h).collect();
        let c = h.iter().next().unwrap();
        res += match c {
            'a'..='z' => (**c as u64) - 96,
            'A'..='Z' => (**c as u64) - 38,
            _ => 0 
        }
    }
    return res;
}

fn part2(s: String) -> u64 {
    let mut res: u64 = 0;
    let b = s.split("\n").map(|x| String::from(x)).collect::<Vec<String>>();
    let it = b.chunks(3);
    for x in it {
        let aa = &x[0];
        let bb = &x[1];
        let cc = &x[2];
        let mut aa_s = HashSet::new();
        let mut bb_s = HashSet::new(); 
        let mut cc_s = HashSet::new();

        for a in aa.chars() {
            aa_s.insert(a);
        }

        for b in bb.chars() {
            bb_s.insert(b);
        }

        for c in cc.chars() {
            cc_s.insert(c);
        }

        let foo: HashSet<char> = aa_s.intersection(&bb_s).map(|x| *x).collect();
        let bar: HashSet<char> = foo.intersection(&cc_s).map(|x| *x).collect();

        let c = bar.iter().next().unwrap();
        res += match c {
            'a'..='z' => (*c as u64) - 96,
            'A'..='Z' => (*c as u64) - 38,
            _ => 0 
        }
    }
    return res;
}

fn main() -> () {
    let s = String::from(include_str!("../inputs/03_in.txt"));
    let t = s.clone();
    println!("{}", part1(s));
    println!("{}", part2(t));
}