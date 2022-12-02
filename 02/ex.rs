use std::fs;

fn parse_part1_input(s: String) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Vec::new();
    let games: Vec<Vec<&str>> = s.split("\n").map(|x| x.split(" ").collect()).collect();
    for game in games {
        let left = game[0];
        let right = game[1];
        let left_choice = match left {
            "A" => String::from("R"),
            "B" => String::from("P"),
            "C" => String::from("S"),
            _ => String::from("F"),
        };

        let right_choice = match right {
            "X" => String::from("R"),
            "Y" => String::from("P"),
            "Z" => String::from("S"),
            _ => String::from("F"),
        };

        output.push((left_choice, right_choice))
    }
    return output;
}

fn parse_part2_input(s: String) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = Vec::new();
    let games: Vec<Vec<&str>> = s.split("\n").map(|x| x.split(" ").collect()).collect();
    for game in games {
        let left = game[0];
        let right = game[1];
        let left_choice_str = match left {
            "A" => "R",
            "B" => "P",
            "C" => "S",
            _ => "F",
        };

        let right_choice = match right {
            "X" => {
                match left_choice_str {
                    "R" => String::from("S"),
                    "P" => String::from("R"),
                    "S" => String::from("P"),
                    _ => String::from("F"),
                }
            }
            "Y" => String::from(left_choice_str),
            "Z" => {
                match left_choice_str {
                    "R" => String::from("P"),
                    "P" => String::from("S"),
                    "S" => String::from("R"),
                    _ => String::from("F"),
                }
            },
            _ => String::from("F"),
        };

        output.push((String::from(left_choice_str), right_choice))
    }
    return output;
}

fn score(o: Vec<(String, String)>) -> u64 {
    let mut s: u64 = 0;
    for (l, r) in o {
        let their_choice = l;
        let your_choice = r;
        s += match (your_choice.as_str(), their_choice.as_str()) {
            ("R", "R") => 1 + 3,
            ("R", "P") => 1 + 0,
            ("R", "S") => 1 + 6,
            ("P", "R") => 2 + 6,
            ("P", "P") => 2 + 3,
            ("P", "S") => 2 + 0,
            ("S", "R") => 3 + 0,
            ("S", "P") => 3 + 6,
            ("S", "S") => 3 + 3,
            _ => 0,
        }
    }
    return s;
}

fn main() -> () {
    let s: String = fs::read_to_string("in.txt").expect("failed to read file");
    let t = s.clone();
    let v1 = parse_part1_input(s);
    println!("{}", score(v1));
    let v2 = parse_part2_input(t);
    println!("{}", score(v2));
}