use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

fn main() {
    let file = File::open("test").unwrap();
    let lines = BufReader::new(file).lines();

    let mut universe = Universe::from(lines);
    universe.print();
    universe.expand(2);

    let expanded = universe.expanded();
    let e_distances = get_sum_distances(expanded);
    let galaxies = universe.galaxies();
    let g_distances = get_sum_distances(galaxies);
    println!("non-expanded distance sum: {}", g_distances);
    println!("expanded distance sum: {}", e_distances);
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
    galaxies: Vec<(isize, isize)>,
    expanded: Vec<(isize, isize)>,
}

impl Universe {
    fn print(&self) {
        p(&self.map);
    }

    fn galaxies(&self) -> Vec<(isize, isize)> {
        return self.galaxies.clone();
    }

    fn expanded(&self) -> Vec<(isize, isize)> {
        return self.expanded.clone();
    }

    fn get_galaxies(&mut self) {
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == '#' {
                    self.galaxies.push((x as isize, y as isize));
                }
            }
        }
        self.expanded = self.galaxies.clone();
    }

    fn expand(&mut self, amount: isize) {
        self.expand_rows(amount);
        self.expand_columns(amount);
    }

    fn expand_columns(&mut self, amount: isize) {
        let mut x = 0;
        while x < self.map[0].len() {
            if !column_contains(x, &self.map, '#') {
                for i in 0..self.galaxies.len() {
                    if self.galaxies[i].0 > x as isize {
                        self.expanded[i].0 += amount;
                    }
                }
            }
            x += 1;
        }
    }

    fn insert_column(&mut self, column: usize, c: char) {
        for y in 0..self.map.len() {
            self.map[y].insert(column, c);
        }
    }

    fn expand_rows(&mut self, amount: isize) {
        let mut y = 0;
        while y < self.map.len() {
            if !self.map[y].contains(&'#') {
                for i in 0..self.galaxies.len() {
                    if self.galaxies[i].1 > y as isize {
                        self.expanded[i].1 += amount;
                    }
                }
            }
            y += 1;
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

        let mut universe = Universe {
            map,
            galaxies: Vec::new(),
            expanded: Vec::new(),
        };
        universe.get_galaxies();
        return universe;
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
