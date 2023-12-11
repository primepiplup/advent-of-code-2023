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

    for mapper in mappers {
        seeds = seeds
            .into_iter()
            .map(|seed| mapper.transform(seed))
            .collect();
    }

    let mut lowest = seeds[0];
    for i in 1..seeds.len() {
        if seeds[i] < lowest {
            lowest = seeds[i];
        }
    }
    println!("{:?}", lowest);
}

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
        for map in self.maps.clone() {
            if input >= map.0 && input <= map.1 {
                return input + map.2;
            }
        }
        return input;
    }
}
