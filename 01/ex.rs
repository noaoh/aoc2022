use std::fs;

fn parse_input(s: String) -> Vec<Vec<u64>> {
    let mut output: Vec<Vec<u64>> = Vec::new();
    let elves = s.split("\n\n");
    for elf in elves {
        let mut o: Vec<u64> = Vec::new(); 
        let foods = elf.split("\n");
        for food in foods {
            let food_num: u64 = food.trim()
                .parse()
                .expect("conversion from string to u64 failed");
            o.push(food_num);
        } 
        output.push(o);
    }
    return output;
}

fn part1(elves_calories: Vec<Vec<u64>>) -> u64 {
    return elves_calories
        .iter()
        .map(|elf_calories| elf_calories.iter().sum())
        .max()
        .expect("should return a number");
} 

fn part2(elves_calories: Vec<Vec<u64>>) -> u64 {
    let mut summed_calories = elves_calories
        .iter()
        .map(|elf_calories| elf_calories.iter().sum())
        .collect::<Vec<u64>>();

    summed_calories.sort_by(|a, b| b.cmp(a));
    let top_three = summed_calories[0..3].to_vec();
    return top_three.iter().sum();
}

fn main() -> () {
    let s: String = fs::read_to_string("in.txt").expect("failed to read file");
    let vec1 = parse_input(s);
    let vec2 = vec1.clone(); 
    println!("{}", part1(vec1));
    println!("{}", part2(vec2));
}
