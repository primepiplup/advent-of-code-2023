use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut field = Field::from(lines);

    let mut collector = Vec::new();
    for i in 0..500 {
        field.move_north();
        field.move_west();
        field.move_south();
        field.move_east();
        let new = field.get_weight();
        collector.push(new);
        println!("Current index {} ... weight {}", i, new);
    }

    let pattern = find_pattern(collector);

    let length = pattern.len();

    let remaining = 1000000000 - 501;
    let index = remaining % length;

    let current = pattern[index];

    println!("Pattern: {:?}", pattern);

    println!("Weight: {}", current);
}

fn find_pattern(nums: Vec<usize>) -> Vec<usize> {
    let mut end = nums.len() - 1;
    let mut pattern = Vec::new();
    pattern.push(nums[end]);
    end -= 1;
    while end > 0 {
        while nums[end] != pattern[0] && end > 0 {
            pattern.push(nums[end]);
            end -= 1;
        }
        if is_pattern(&nums, &pattern, end) {
            break;
        } else {
            pattern.push(nums[end]);
            end -= 1;
        }
    }

    pattern.reverse();
    return pattern;
}

fn is_pattern(nums: &Vec<usize>, pattern: &Vec<usize>, end: usize) -> bool {
    if pattern.len() < 2 {
        return false;
    }
    for i in 0..pattern.len() {
        if nums[end - i] != pattern[i] {
            return false;
        }
    }
    return true;
}

#[derive(Debug)]
struct Field {
    field: Vec<Vec<char>>,
}

impl From<Lines<BufReader<File>>> for Field {
    fn from(lines: Lines<BufReader<File>>) -> Field {
        let mut field = Vec::new();
        for line in lines {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };

            field.push(line.chars().collect());
        }
        return Field { field };
    }
}

impl Field {
    fn move_north(&mut self) -> () {
        while self.north_tick() {}
    }

    fn north_tick(&mut self) -> bool {
        let mut change = false;
        for y in 1..self.field.len() {
            for x in 0..self.field[y].len() {
                if self.field[y][x] == 'O' && self.field[y - 1][x] == '.' {
                    self.field[y][x] = '.';
                    self.field[y - 1][x] = 'O';
                    change = true;
                }
            }
        }
        return change;
    }

    fn move_south(&mut self) -> () {
        while self.south_tick() {}
    }

    fn south_tick(&mut self) -> bool {
        let mut change = false;
        let mut y = self.field.len() - 2;
        while y != usize::MAX {
            for x in 0..self.field[y].len() {
                if self.field[y][x] == 'O' && self.field[y + 1][x] == '.' {
                    self.field[y][x] = '.';
                    self.field[y + 1][x] = 'O';
                    change = true;
                }
            }
            y = y.wrapping_sub(1);
        }
        return change;
    }

    fn move_east(&mut self) -> () {
        while self.east_tick() {}
    }

    fn east_tick(&mut self) -> bool {
        let mut change = false;
        let mut x = self.field[0].len() - 2;
        while x != usize::MAX {
            for y in 0..self.field[x].len() {
                if self.field[y][x] == 'O' && self.field[y][x + 1] == '.' {
                    self.field[y][x] = '.';
                    self.field[y][x + 1] = 'O';
                    change = true;
                }
            }
            x = x.wrapping_sub(1);
        }
        return change;
    }

    fn move_west(&mut self) -> () {
        while self.west_tick() {}
    }

    fn west_tick(&mut self) -> bool {
        let mut change = false;
        for x in 1..self.field[0].len() {
            for y in 0..self.field.len() {
                if self.field[y][x] == 'O' && self.field[y][x - 1] == '.' {
                    self.field[y][x] = '.';
                    self.field[y][x - 1] = 'O';
                    change = true;
                }
            }
        }
        return change;
    }

    fn get_weight(&self) -> usize {
        let mut weight = 0;
        let mut weight_multiplier = self.field.len();
        for line in &self.field {
            for c in line {
                if c == &'O' {
                    weight += weight_multiplier;
                }
            }
            weight_multiplier -= 1;
        }
        return weight;
    }

    fn print(&self) -> () {
        for line in &self.field {
            for c in line {
                print!("{}", c);
            }
            print!("\n");
        }
        print!("\n");
    }
}
