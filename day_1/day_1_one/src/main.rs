use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input").unwrap();
    let lines = io::BufReader::new(file).lines();

    let num_chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let num_words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];

    let mut total = 0;
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        let nums = get_numbers_from_line(line, &num_chars, &num_words);
        total += get_value(nums);
    }

    println!("{}", total);
}

fn get_numbers_from_line(line: String, nums: &Vec<char>, num_words: &Vec<&str>) -> Vec<char> {
    let mut collector: Vec<char> = Vec::new();
    let mut line = line.to_lowercase();
    while line != "" {
        for i in 0..nums.len() {
            if line.starts_with(nums[i]) {
                collector.push(nums[i]);
            }
            if line.starts_with(num_words[i]) {
                collector.push(nums[i]);
            }
        }
        line.remove(0);
    }
    return collector;
}

fn get_value(nums: Vec<char>) -> usize {
    if nums.len() == 0 {
        return 0;
    }
    let first = nums[0];
    let last = nums[nums.len() - 1];
    format!("{}{}", first, last).parse().unwrap()
}
