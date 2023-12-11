use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};

fn main() {
    let input = File::open("input").unwrap();
    let mut readbuffer = io::BufReader::new(input).lines();

    let seeds_line = readbuffer.next().unwrap().unwrap();
    readbuffer.next(); // discard empty line

    let mut seeds = parse_seeds(seeds_line);

    let mut mappers: Vec<Mapper> = Vec::new();
    for _ in 0..7 {
        mappers.push(get_map(&mut readbuffer))
    }

    let mut lowest = seeds[0];
    for i in (0..seeds.len()).step_by(2) {
        let mut location = seeds[i];
        let end = location + seeds[i + 1];
        println!("{}, {}", location, end);
        while location < end {
            let mut seed = location;
            for mapper in &mappers {
                seed = mapper.transform(seed);
            }
            if seed < lowest {
                lowest = seed;
            }
            location += 1;
        }
    }
    println!("{:?}", lowest);
}

// fn range_seeds(seeds: Vec<isize>) -> Vec<isize> {
//     let mut seed_ranges = Vec::new();
//     for i in (0..seeds.len()).step_by(2) {
//         let begin = seeds[i];
//         let length = seeds[i + 1];
//         let mut range: Vec<isize> = (begin..(begin + length)).collect();
//         seed_ranges.append(&mut range);
//     }
//     return seed_ranges;
// }

fn parse_seeds(seeds_line: String) -> Vec<isize> {
    seeds_line.split(":").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|seed| seed.parse().unwrap())
        .collect()
}

fn get_map(buffer: &mut Lines<BufReader<File>>) -> Mapper {
    let mut mapper = Mapper::new();
    mapper.parse_map_names(buffer.next().unwrap().unwrap());
    let mut line = buffer.next().unwrap().unwrap();
    while !line.is_empty() {
        mapper.parse_mapping(line);
        line = buffer.next().unwrap().unwrap();
    }
    return mapper;
}

#[derive(Debug, Clone)]
struct Mapper {
    input: String,
    output: String,
    maps: Vec<(isize, isize, isize)>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper {
            input: "".to_string(),
            output: "".to_string(),
            maps: Vec::new(),
        }
    }

    fn parse_mapping(&mut self, line: String) -> () {
        let mapping: Vec<isize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let from = mapping[1];
        let to = mapping[1] + mapping[2] - 1;
        let difference = mapping[0] - mapping[1];
        println!("from {} to {} with difference {}", from, to, difference);
        self.maps.push((from, to, difference));
    }

    fn parse_map_names(&mut self, line: String) -> () {
        let names: Vec<String> = line.split_whitespace().collect::<Vec<&str>>()[0]
            .split("-to-")
            .map(|word| word.to_owned())
            .collect();
        println!("{}-to-{}", names[0], names[1]);
        self.input = names[0].clone();
        self.output = names[1].clone();
    }

    fn transform(&self, input: isize) -> isize {
        for map in &self.maps {
            if input >= map.0 && input <= map.1 {
                return input + map.2;
            }
        }
        return input;
    }
}
