use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();

    let mut universe = Universe::from(lines);
    universe.print();
    universe.expand();
    universe.print();

    let galaxies = universe.galaxies();
    let distances = get_sum_distances(galaxies);
    println!("distance sum: {}", distances);
}

fn get_sum_distances(galaxies: Vec<(isize, isize)>) -> isize {
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            total += get_galaxy_distance(galaxies[i], galaxies[j]);
        }
    }
    return total;
}

fn get_galaxy_distance(g1: (isize, isize), g2: (isize, isize)) -> isize {
    let dx = (g1.0 - g2.0).abs();
    let dy = (g1.1 - g2.1).abs();
    let distance = dx + dy;
    return distance;
}

struct Universe {
    map: Vec<Vec<char>>,
}

impl Universe {
    fn print(&self) {
        p(&self.map);
    }

    fn galaxies(&self) -> Vec<(isize, isize)> {
        let mut galaxies = Vec::new();
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == '#' {
                    galaxies.push((x as isize, y as isize));
                }
            }
        }
        return galaxies;
    }

    fn expand(&mut self) {
        self.expand_columns();
        self.expand_rows();
    }

    fn expand_rows(&mut self) {
        let mut x = 0;
        while x < self.map[0].len() {
            if !column_contains(x, &self.map, '#') {
                self.insert_column(x, '.');
                x += 1;
            }
            x += 1;
        }
    }

    fn insert_column(&mut self, column: usize, c: char) {
        for y in 0..self.map.len() {
            self.map[y].insert(column, c);
        }
    }

    fn expand_columns(&mut self) {
        let mut i = 0;
        let width = self.map[0].len();
        while i < self.map.len() {
            if !self.map[i].contains(&'#') {
                self.map.insert(i, Vec::new());
                for _ in 0..width {
                    self.map[i].push('.');
                }
                i += 1;
            }
            i += 1;
        }
    }
}

impl From<Lines<BufReader<File>>> for Universe {
    fn from(buffer: Lines<BufReader<File>>) -> Universe {
        let mut map = Vec::new();

        for line in buffer {
            let line = match line {
                Ok(line) => line,
                Err(_) => break,
            };
            map.push(line_to_vec(line));
        }

        Universe { map }
    }
}

fn line_to_vec(line: String) -> Vec<char> {
    line.chars().collect()
}

fn p(map: &Vec<Vec<char>>) -> () {
    for row in map {
        for c in row {
            print!("{}", c);
        }
        print!("\n");
    }
    print!("\n");
}

fn column_contains(column: usize, map: &Vec<Vec<char>>, c: char) -> bool {
    for y in 0..map.len() {
        if map[y][column] == c {
            return true;
        }
    }
    return false;
}
