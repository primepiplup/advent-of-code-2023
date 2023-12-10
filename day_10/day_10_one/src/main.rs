use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

static EAST: [char; 3] = ['-', 'J', '7'];
static NORTH: [char; 3] = ['|', 'F', '7'];
static WEST: [char; 3] = ['-', 'L', 'F'];
static SOUTH: [char; 3] = ['|', 'J', 'L'];

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let field = read_field(lines);
    dbg!("{}", field.clone());
    let distance = find_longest_distance_in_field(field);
    println!("distance: {}", distance);
}

fn find_longest_distance_in_field(field: Vec<Vec<char>>) -> usize {
    let (x, y) = find_location(field.clone());

    let mut distance = 0;
    if NORTH.contains(&field[y - 1][x]) {
        distance = walk(field, (x, y - 1), Direction::North);
    } else if SOUTH.contains(&field[y + 1][x]) {
        distance = walk(field, (x, y + 1), Direction::South);
    } else if EAST.contains(&field[y][x + 1]) {
        distance = walk(field, (x + 1, y), Direction::East);
    } else if WEST.contains(&field[y][x - 1]) {
        distance = walk(field, (x - 1, y), Direction::West);
    }
    return distance;
}

fn walk(field: Vec<Vec<char>>, location: (usize, usize), direction: Direction) -> usize {
    println!(
        "starting at: {:?} with direction: {:?}",
        location, direction
    );
    let mut agent = Agent::new(direction, location);
    let mut distance = 1;
    let (mut x, mut y) = agent.location();
    while field[y][x] != 'S' {
        println!("location: x: {}, y: {}, pipe: {}", x, y, field[y][x]);
        agent.crawl(field[y][x]);
        (x, y) = agent.location();
        distance += 1;
    }
    return distance;
}

fn find_location(field: Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in field.into_iter().enumerate() {
        for (x, c) in row.into_iter().enumerate() {
            if c == 'S' {
                return (x, y);
            }
        }
    }
    return (0, 0);
}

struct Agent {
    direction: Direction,
    location: (usize, usize),
}

impl Agent {
    fn new(direction: Direction, location: (usize, usize)) -> Agent {
        Agent {
            direction,
            location,
        }
    }

    fn location(&self) -> (usize, usize) {
        self.location
    }

    fn crawl(&mut self, pipe: char) -> () {
        match pipe {
            '|' => {
                if self.direction == Direction::North {
                    self.location.1 -= 1;
                } else {
                    self.location.1 += 1;
                }
            }
            '-' => {
                if self.direction == Direction::East {
                    self.location.0 += 1;
                } else {
                    self.location.0 -= 1;
                }
            }
            'L' => {
                if self.direction == Direction::West {
                    self.direction = Direction::North;
                    self.location.1 -= 1;
                } else {
                    self.direction = Direction::East;
                    self.location.0 += 1;
                }
            }
            'J' => {
                if self.direction == Direction::East {
                    self.direction = Direction::North;
                    self.location.1 -= 1;
                } else {
                    self.direction = Direction::West;
                    self.location.0 -= 1;
                }
            }
            '7' => {
                if self.direction == Direction::East {
                    self.direction = Direction::South;
                    self.location.1 += 1;
                } else {
                    self.direction = Direction::West;
                    self.location.0 -= 1;
                }
            }
            'F' => {
                if self.direction == Direction::North {
                    self.direction = Direction::East;
                    self.location.0 += 1;
                } else {
                    self.direction = Direction::South;
                    self.location.1 += 1;
                }
            }
            _ => {}
        }
    }
}

fn read_field(lines: Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut field = Vec::new();
    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };
        field.push(read_line(line));
    }
    return field;
}

fn read_line(line: String) -> Vec<char> {
    line.chars().collect()
}

#[derive(PartialEq, Eq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}
