use std::fs;
use std::io::{self, BufRead};

fn main() {
    let collected: Vec<(Vec<u32>, Vec<u32>)> = collect_input_into_vec("input");

    let mut total = 0;
    for (winners, nums) in collected {
        total += get_card_score(winners, nums);
    }
    println!("{}", total);
}

fn collect_input_into_vec(input_file: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let input = fs::File::open(input_file).expect("could not open input file");
    let lines = io::BufReader::new(input).lines();
    let mut collected: Vec<(Vec<u32>, Vec<u32>)> = Vec::new();
    for possible_line in lines {
        if let Ok(line) = possible_line {
            let numbers: &str = line.split(":").collect::<Vec<&str>>()[1];
            let numbers_sep: Vec<&str> = numbers.split("|").collect();
            let winners = string_to_vec_of_int(numbers_sep[0]);
            let possibles = string_to_vec_of_int(numbers_sep[1]);
            collected.push((winners, possibles));
        }
    }
    return collected;
}

fn string_to_vec_of_int(numbers: &str) -> Vec<u32> {
    numbers
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn get_card_score(winners: Vec<u32>, possibles: Vec<u32>) -> u32 {
    let mut counter = 0;
    let mut score = 0;
    for num in possibles {
        for winner in winners.clone() {
            if num == winner {
                counter += 1;
            }
        }
    }
    if counter > 0 {
        counter -= 1;
        score += 1;
    }
    while counter > 0 {
        counter -= 1;
        score *= 2;
    }
    return score;
}
