use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut collector: Vec<Game> = Vec::new();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        collector.push(Game::from(line))
    }

    let mut counter = 0;
    for (_, game) in collector.into_iter().enumerate() {
        counter += game.minimum_power();
    }

    println!("{}", counter);
}

#[derive(Debug)]
struct Game {
    rounds: Vec<Set>,
}

impl Game {
    fn new() -> Game {
        Game { rounds: Vec::new() }
    }

    fn add(&mut self, set: Set) -> () {
        self.rounds.push(set);
    }

    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        for round in self.rounds.to_owned() {
            if !round.is_possible(red, green, blue) {
                return false;
            }
        }
        return true;
    }

    fn minimum_power(&self) -> usize {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in self.rounds.to_owned() {
            if round.red > max_red {
                max_red = round.red;
            }
            if round.green > max_green {
                max_green = round.green;
            }
            if round.blue > max_blue {
                max_blue = round.blue;
            }
        }
        return max_red * max_green * max_blue;
    }
}

impl From<String> for Game {
    fn from(line: String) -> Game {
        let mut game = Game::new();
        let sets: Vec<&str> = line.split(":").collect::<Vec<&str>>()[1]
            .split(";")
            .collect();
        for set in sets {
            let set = Set::from(set);
            game.add(set);
        }

        return game;
    }
}

#[derive(Debug, Clone)]
struct Set {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Set {
    fn new() -> Set {
        Set {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        if red < self.red {
            return false;
        } else if green < self.green {
            return false;
        } else if blue < self.blue {
            return false;
        } else {
            return true;
        }
    }

    fn parse_colors(&mut self, color: &str) -> () {
        let color: Vec<&str> = color.split_whitespace().collect();
        if color[1] == "red" {
            self.red = color[0].parse().unwrap();
        } else if color[1] == "green" {
            self.green = color[0].parse().unwrap();
        } else if color[1] == "blue" {
            self.blue = color[0].parse().unwrap();
        }
    }
}

impl From<&str> for Set {
    fn from(set_line: &str) -> Set {
        let mut set = Set::new();

        let colors: Vec<&str> = set_line.split(",").collect();
        for color in colors {
            set.parse_colors(color);
        }

        return set;
    }
}
