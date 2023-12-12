use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let rows = get_spring_rows(lines);

    let mut possibilities = 0;
    for row in rows {
        let possibles = row.possibles();
        possibilities += possibles;
        println!("Found: {}", possibles);
        println!("");
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
        return Row { info, groups }.expand();
    }
}

impl Row {
    fn possibles(&self) -> usize {
        println!("Running: {:?}", self.info);
        let possibles = fill_group_recurse(
            self.info.clone(),
            self.groups.clone(),
            self.groups.clone(),
            0,
        );
        return possibles;
    }

    fn expand(self) -> Row {
        let mut info = Vec::new();
        for _ in 0..5 {
            for c in self.info.clone() {
                info.push(c);
            }
            info.push('?');
        }

        let mut groups = Vec::new();
        for _ in 0..5 {
            for n in self.groups.clone() {
                groups.push(n);
            }
        }
        Row { info, groups }
    }
}

fn fill_group_recurse(
    line: Vec<char>,
    to_fill: Vec<usize>,
    groups: Vec<usize>,
    start: usize,
) -> usize {
    if early_fail(&line, &to_fill, &groups) {
        return 0;
    }

    let mut line = line;
    if to_fill.len() == 0 {
        let filled = count_row(&line);
        if filled == groups {
            return 1;
        } else {
            return 0;
        }
    }

    let mut total = 0;
    for i in start..line.len() {
        if line[i] == '?' || line[i] == '#' {
            let mut possible = line.clone();

            if apply_group(&mut possible, to_fill[0], i) {
                let mut to_fill_minus_one = to_fill.clone();
                to_fill_minus_one.remove(0);
                total +=
                    fill_group_recurse(possible, to_fill_minus_one, groups.clone(), i + to_fill[0]);
            }
        }
        if line[i] == '?' {
            line[i] = '.';
        }
    }
    return total;
}

fn early_fail(line: &Vec<char>, left: &Vec<usize>, groups: &Vec<usize>) -> bool {
    let filled_in = groups.len() - left.len();
    if filled_in == 0 {
        return false;
    }
    let current = count_row(line);
    for i in 0..filled_in {
        if current[i] != groups[i] {
            return true;
        }
    }
    return false;
}

fn apply_group(possible: &mut Vec<char>, size: usize, location: usize) -> bool {
    let limit = possible.len();
    let end = location + size;

    if end > limit {
        return false;
    }

    if end != limit {
        if possible[end] == '#' {
            return false;
        }
        possible[end] = '.';
    }

    for i in location..end {
        if possible[i] != '.' {
            possible[i] = '#';
        } else {
            return false;
        }
    }

    return true;
}

fn count_row(line: &Vec<char>) -> Vec<usize> {
    let mut collector = Vec::new();
    let mut counter = 0;
    for c in line {
        if c == &'#' {
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

// fn get_missing(&self) -> usize {
//     let mut needed = 0;
//     self.groups
//         .clone()
//         .into_iter()
//         .for_each(|num| needed += num);
//     let mut present = 0;
//     self.info.clone().into_iter().for_each(|c| {
//         if c == '#' {
//             present += 1
//         }
//     });
//     let missing = needed - present;
//     return missing;
// }
