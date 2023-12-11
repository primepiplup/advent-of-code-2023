use std::fs;
use std::io::{self, BufRead};

fn main() {
    let collected: Vec<(Vec<usize>, Vec<usize>)> = collect_input_into_vec("input");

    let scored_cards: Vec<usize> = score_cards(collected);
    dbg! {"{}", scored_cards.clone()};

    let mut total = 0;
    for i in 0..scored_cards.len() {
        total += collect_copies(i, scored_cards.clone());
    }
    println!("{}", total);
}

fn score_cards(collected: Vec<(Vec<usize>, Vec<usize>)>) -> Vec<usize> {
    let mut collector: Vec<usize> = Vec::new();
    for (winners, possibles) in collected {
        collector.push(get_card_score(winners, possibles));
    }
    return collector;
}

fn collect_copies(i: usize, scored: Vec<usize>) -> usize {
    let score = match scored.get(i) {
        Some(val) => val.to_owned(),
        None => return 0,
    };
    if score == 0 {
        return 1;
    }
    let mut copies = 1;
    for j in 1..=score {
        // if i + j > scored.len() {
        //     return copies;
        // }
        copies += collect_copies(i + j, scored.clone());
    }
    return copies;
}

fn collect_input_into_vec(input_file: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let input = fs::File::open(input_file).expect("could not open input file");
    let lines = io::BufReader::new(input).lines();
    let mut collected: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
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

fn string_to_vec_of_int(numbers: &str) -> Vec<usize> {
    numbers
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn get_card_score(winners: Vec<usize>, possibles: Vec<usize>) -> usize {
    let mut counter = 0;
    for num in possibles {
        for winner in winners.clone() {
            if num == winner {
                counter += 1;
            }
        }
    }
    return counter;
}
