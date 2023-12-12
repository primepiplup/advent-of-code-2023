use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let rows = get_spring_rows(lines);

    let mut possibilities = 0;
    for row in rows {
        possibilities += row.possibles();
    }

    println!("{}", possibilities);
}

fn get_spring_rows(buffer: Lines<BufReader<File>>) -> Vec<Row> {
    let mut rows = Vec::new();
    for line in buffer {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        rows.push(Row::from(line));
    }
    return rows;
}

#[derive(Debug, Clone)]
struct Row {
    info: Vec<char>,
    groups: Vec<usize>,
}

impl From<String> for Row {
    fn from(line: String) -> Row {
        let split: Vec<&str> = line.split_whitespace().collect();
        let info: Vec<char> = split[0].chars().collect();
        let groups: Vec<usize> = split[1]
            .split(",")
            .map(|num| num.parse().unwrap())
            .collect();
        Row { info, groups }
    }
}

impl Row {
    fn possibles(&self) -> usize {
        let missing = self.get_missing();
        println!("Running: {:?} with {} missing", self.info, missing);
        let possibles = fill_one_recurse(self.info.clone(), self.groups.clone(), missing);
        return possibles;
    }

    fn get_missing(&self) -> usize {
        let mut needed = 0;
        self.groups
            .clone()
            .into_iter()
            .for_each(|num| needed += num);
        let mut present = 0;
        self.info.clone().into_iter().for_each(|c| {
            if c == '#' {
                present += 1
            }
        });
        let missing = needed - present;
        return missing;
    }
}

fn fill_one_recurse(line: Vec<char>, groups: Vec<usize>, missing: usize) -> usize {
    if missing == 0 {
        let filled = count_row(line.clone());
        println!("{:?}, filled: {:?}, groups: {:?}", line, filled, groups);
        if filled == groups {
            println!("Correct!");
            return 1;
        } else {
            println!("Incorrect!");
            return 0;
        }
    }

    let mut total = 0;
    for i in 0..line.len() {
        if line[i] == '?' {
            let mut possible = line.clone();
            possible[i] = '#';
            for j in 0..i {
                if possible[j] == '?' {
                    possible[j] = '.';
                }
            }
            total += fill_one_recurse(possible, groups.clone(), missing - 1);
        }
    }
    return total;
}

fn count_row(line: Vec<char>) -> Vec<usize> {
    let mut collector = Vec::new();
    let mut counter = 0;
    for c in line {
        if c == '#' {
            counter += 1;
        } else if counter > 0 {
            collector.push(counter);
            counter = 0;
        }
    }
    if counter > 0 {
        collector.push(counter);
    }
    return collector;
}
