use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::fs::File;
use std::fmt;

#[derive(Debug)]
struct Map {
    idim: usize,  // vertical
    jdim: usize,  // horizontal
    map: Vec<Vec<bool>>
}

impl Map {
    fn get(&self, i: usize, j: usize) -> Option<bool> {
        let jj = j % self.jdim;
        if i < self.idim {
            Some(self.map[i][jj])
        } else {
            None
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} x {})\n", self.idim, self.jdim)?;
        let mut buf = String::new();
        for row in &self.map {
            let mut line = String::new();
            for c in row {
                line.push(
                    match c {
                        true => '#',
                        false => '.'
                    }
                    );
            }
            line.push('\n');
            buf.push_str(&line);
        }
        write!(f, "{}", buf)
    }
}

fn traverse(map: &Map, di: usize, dj: usize) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut count = 0;
    loop {
        i += di;
        j += dj;
        let c = match map.get(i, j) {
            Some(true) => {
                count += 1;
                true
            },
            Some(false) => {
                false
            },
            None => break
        };
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let map = read_map(filename).unwrap();
    println!("{}", map);

    let mut count: usize = 0;
    let mut prod: usize = 1;

    let steps: Vec<(usize, usize)> = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    for (di, dj) in steps {
        count = traverse(&map, di, dj);
        println!("count = {count}");
        prod *= count;
    }
    println!("prod = {prod}");
}

fn read_map(filename: &str) -> Result<Map, std::io::Error> {
    let f = File::open(filename)?;
    let mut map = Map{idim: 0, jdim: 0, map: Vec::new()};

    let reader = BufReader::new(f);
    let mut first = true;
    for (_i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if first {
            map.jdim = line.trim().len();
            first = false;
        }
        let mut vec_h = vec![false; map.jdim];
        for (j, c) in line.chars().enumerate() {
            vec_h[j] = c == '#';
        }
        map.map.push(vec_h);
        map.idim += 1;
    }

    Ok(map)
}
