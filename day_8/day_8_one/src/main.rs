use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let mut lines = BufReader::new(file).lines();

    let moves = lines.next().unwrap().unwrap();
    let mut moves = Moves::new(moves);
    lines.next();

    let nodes = read_nodes(lines);

    let mut current = "AAA".to_owned();
    let mut counter = 0;
    while current != "ZZZ" {
        let c = moves.next();
        let options = nodes.get(&current).unwrap();
        if c == 'L' {
            current = options.0.clone();
        } else {
            current = options.1.clone();
        }
        counter += 1;
    }
    println!("{}", counter);
}

fn read_nodes(buffer: Lines<BufReader<File>>) -> HashMap<String, (String, String)> {
    let mut map = HashMap::new();
    for line in buffer {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        let (node, paths) = read_node(line);
        map.insert(node, paths);
    }
    return map;
}

fn read_node(line: String) -> (String, (String, String)) {
    let split: Vec<&str> = line.split("=").collect();
    let node = split[0].trim().to_owned();
    let paths = split[1].replace("(", "").replace(")", "");
    let mut paths = paths.split(",").map(|path| path.trim().to_owned());
    let left = paths.next().unwrap();
    let right = paths.next().unwrap();
    return (node, (left, right));
}

struct Moves {
    moves: Vec<char>,
    counter: usize,
}

impl Moves {
    fn new(line: String) -> Moves {
        Moves {
            moves: line.chars().collect(),
            counter: 0,
        }
    }

    fn next(&mut self) -> char {
        let c = self.moves[self.counter];
        self.counter += 1;
        if self.counter >= self.moves.len() {
            self.counter = 0;
        }
        return c;
    }
}
