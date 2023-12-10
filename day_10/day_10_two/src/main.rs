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
    let mut notes = field.clone();
    empty_notes(&mut notes);
    let surface = find_loop_surface(&field, &mut notes);
    p(&field);
    p(&notes);
    println!("surface: {}", surface);
}

fn p(field: &Vec<Vec<char>>) -> () {
    for row in field {
        for c in row {
            print!("{}", c);
        }
        print!("\n");
    }
}

fn empty_notes(notes: &mut Vec<Vec<char>>) -> () {
    for y in 0..notes.len() {
        for x in 0..notes[y].len() {
            notes[y][x] = '.';
        }
    }
}

fn mark(notes: &mut Vec<Vec<char>>, (x, y): (usize, usize), marker: char) -> () {
    notes[y][x] = marker;
}

fn find_loop_surface(field: &Vec<Vec<char>>, notes: &mut Vec<Vec<char>>) -> usize {
    let (x, y) = find_location(field.clone());
    mark(notes, (x, y), 'x');

    if NORTH.contains(&field[y - 1][x]) {
        _ = walk(field, (x, y - 1), Direction::North, notes);
        p(&field);
        p(&notes);
        double_time(field, (x, y - 1), Direction::North, notes);
    } else if EAST.contains(&field[y][x + 1]) {
        _ = walk(field, (x + 1, y), Direction::East, notes);
        p(&field);
        p(&notes);
        double_time(field, (x + 1, y), Direction::East, notes);
    } else if SOUTH.contains(&field[y + 1][x]) {
        _ = walk(field, (x, y + 1), Direction::South, notes);
        p(&field);
        p(&notes);
        double_time(field, (x, y + 1), Direction::South, notes);
    }
    if WEST.contains(&field[y][x - 1]) {
        _ = walk(field, (x - 1, y), Direction::West, notes);
        p(&field);
        p(&notes);
        double_time_left(field, (x - 1, y), Direction::West, notes);
    }

    let total = count_marks(notes);

    return total;
}

fn count_marks(notes: &mut Vec<Vec<char>>) -> usize {
    let mut counter = 0;
    for row in notes {
        for c in row {
            if *c == 'M' {
                counter += 1;
            }
        }
    }
    return counter;
}

fn double_time(
    field: &Vec<Vec<char>>,
    location: (usize, usize),
    direction: Direction,
    notes: &mut Vec<Vec<char>>,
) -> () {
    println!(
        "starting at: {:?} with direction: {:?}",
        location, direction
    );
    let mut agent = Agent::new(direction, location);
    agent.clone().shoot_right(notes);
    let (mut x, mut y) = agent.location();
    while field[y][x] != 'S' {
        mark(notes, (x, y), 'x');
        println!("location: x: {}, y: {}, pipe: {}", x, y, field[y][x]);
        agent.crawl(field[y][x]);
        (x, y) = agent.location();
        agent.clone().shoot_right(notes);
    }
}

fn double_time_left(
    field: &Vec<Vec<char>>,
    location: (usize, usize),
    direction: Direction,
    notes: &mut Vec<Vec<char>>,
) -> () {
    println!(
        "starting at: {:?} with direction: {:?}",
        location, direction
    );
    let mut agent = Agent::new(direction, location);
    agent.clone().shoot_left(notes);
    let (mut x, mut y) = agent.location();
    while field[y][x] != 'S' {
        mark(notes, (x, y), 'x');
        println!("location: x: {}, y: {}, pipe: {}", x, y, field[y][x]);
        agent.crawl(field[y][x]);
        (x, y) = agent.location();
        agent.clone().shoot_left(notes);
    }
}

fn walk(
    field: &Vec<Vec<char>>,
    location: (usize, usize),
    direction: Direction,
    notes: &mut Vec<Vec<char>>,
) -> Vec<(usize, usize)> {
    println!(
        "starting at: {:?} with direction: {:?}",
        location, direction
    );
    let mut agent = Agent::new(direction, location);
    let mut locations = Vec::new();
    let (mut x, mut y) = agent.location();
    locations.push((x, y));
    while field[y][x] != 'S' {
        mark(notes, (x, y), 'x');
        println!("location: x: {}, y: {}, pipe: {}", x, y, field[y][x]);
        agent.crawl(field[y][x]);
        (x, y) = agent.location();
        locations.push((x, y));
    }
    return locations;
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

#[derive(Debug, Clone)]
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

    fn shoot_right(&mut self, notes: &mut Vec<Vec<char>>) -> () {
        loop {
            if self.direction == Direction::North {
                self.location.0 += 1;
            } else if self.direction == Direction::East {
                self.location.1 += 1;
            } else if self.direction == Direction::South {
                self.location.0 -= 1;
            } else if self.direction == Direction::West {
                self.location.1 -= 1;
            }
            if notes[self.location.1][self.location.0] == 'x' {
                break;
            }
            mark(notes, self.location, 'M');
        }
    }

    fn shoot_left(&mut self, notes: &mut Vec<Vec<char>>) -> () {
        loop {
            if self.direction == Direction::North {
                self.location.0 -= 1;
            } else if self.direction == Direction::East {
                self.location.1 -= 1;
            } else if self.direction == Direction::South {
                self.location.0 += 1;
            } else if self.direction == Direction::West {
                self.location.1 += 1;
            }
            if notes[self.location.1][self.location.0] == 'x' {
                break;
            }
            mark(notes, self.location, 'M');
        }
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}
