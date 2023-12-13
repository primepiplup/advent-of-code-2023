use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let blocks: Vec<Vec<String>> = get_blocks(lines);

    let mut total = 0;
    for block in blocks {
        let area = Area::from(block);
        total += area.get_mirror();
    }

    println!("{}", total);
}

#[derive(Debug)]
struct Area {
    horizontal: Vec<u32>,
    vertical: Vec<u32>,
}

impl From<Vec<String>> for Area {
    fn from(lines: Vec<String>) -> Area {
        let horizontal = Area::get_horizontal(&lines);
        let vertical = Area::get_vertical(&lines);

        Area {
            horizontal,
            vertical,
        }
    }
}

impl Area {
    fn get_mirror(&self) -> u32 {
        let horizontal = Self::mirror(&self.horizontal) * 100;
        let vertical = Self::mirror(&self.vertical);
        return horizontal + vertical;
    }

    fn mirror(nums: &Vec<u32>) -> u32 {
        let length = nums.len();
        for i in 1..length {
            if nums[i - 1] == nums[i] {
                let mut left = i - 1;
                let mut right = i;
                let split = right;
                while left != usize::MAX && right < length {
                    if nums[left] == nums[right] {
                        left = left.wrapping_sub(1);
                        right += 1;
                    } else {
                        break;
                    }
                }
                if left == usize::MAX || right == length {
                    return split as u32;
                }
            }
        }
        return 0;
    }

    fn get_horizontal(lines: &Vec<String>) -> Vec<u32> {
        let mut nums = Vec::new();
        for line in lines {
            let num = convert_chars_to_num(line.chars().collect());
            nums.push(num);
        }
        return nums;
    }

    fn get_vertical(lines: &Vec<String>) -> Vec<u32> {
        let flipped = flip_strings(lines);
        let mut nums = Vec::new();
        for line in flipped {
            println!("{:?}", line);
            let num = convert_chars_to_num(line);
            nums.push(num);
        }
        println!("");
        return nums;
    }
}

fn flip_strings(lines: &Vec<String>) -> Vec<Vec<char>> {
    let block: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let mut collector = Vec::new();
    for x in 0..block[0].len() {
        let mut line = Vec::new();
        for y in 0..block.len() {
            line.push(block[y][x]);
        }
        collector.push(line);
        line = Vec::new();
    }
    return collector;
}

fn convert_chars_to_num(line: Vec<char>) -> u32 {
    let mut num = 0;
    let mut adder = 0;
    for c in line {
        if c == '.' {
            adder = 0;
        } else if c == '#' {
            adder = 1;
        }
        num = num + adder;
        num = num << 1;
    }
    num = num >> 1;
    return num;
}

fn get_blocks(lines: Lines<BufReader<File>>) -> Vec<Vec<String>> {
    let mut blocks: Vec<Vec<String>> = Vec::new();
    let mut block: Vec<String> = Vec::new();

    for line in lines {
        let line = match line {
            Ok(line) => line,
            Err(_) => break,
        };

        if line.contains('.') || line.contains('#') {
            block.push(line);
        } else {
            blocks.push(block);
            block = Vec::new();
        }
    }
    blocks.push(block);
    return blocks;
}
