use std::env;
use std::io::prelude::*;
use std::io;
use std::fs::File;

#[derive(Debug)]
struct PasswordEntry {
    min: usize,
    max: usize,
    c: char,
    password: String
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let mut count: usize = 0;
        for c in self.password.chars() {
            if c == self.c {
                count += 1;
            }
        }
        self.min <= count && count <= self.max
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let entries = read_password(filename).unwrap();

    let mut count = 0;
    for entry in entries {
        let is_valid = entry.is_valid();
        println!("{:?} -> {}", entry, is_valid);
        if is_valid {
            count += 1;
        }
    }
    println!("count = {count}");
}

fn read_password(filename: &str) -> Result<Vec<PasswordEntry>, io::Error> {
    let f = File::open(filename)?;
    let mut reader = io::BufReader::new(f);

    let mut entries: Vec<PasswordEntry> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let tokens: Vec<&str> = line.split(' ').collect();
        let token1: Vec<&str> = tokens[0].split('-').collect();
        let min: usize = token1[0].parse().unwrap();
        let max: usize = token1[1].parse().unwrap();
        let c: char = tokens[1].trim().chars().next().unwrap(); 
        let password: &str = tokens[2].trim();
        entries.push(PasswordEntry { min, max, c, password: password.to_string() });
    }

    Ok(entries)
}
