use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input").expect("file could not be found");
    let mut lines = io::BufReader::new(file).lines();
    let time_line = lines.next().unwrap().unwrap();
    let distance_line = lines.next().unwrap().unwrap();
    let times = get_nums(time_line);
    let distances = get_nums(distance_line);

    let mut total = 1;
    for i in 0..times.len() {
        total *= wintimes(times[i], distances[i]);
    }
    println!("{}", total);
}

fn wintimes(time: usize, distance: usize) -> usize {
    let mut count = 0;
    for dt in 1..time {
        let tl = time - dt;
        let travel = tl * dt;
        if travel > distance {
            count += 1;
        }
    }
    return count;
}

fn get_nums(line: String) -> Vec<usize> {
    line.split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}
