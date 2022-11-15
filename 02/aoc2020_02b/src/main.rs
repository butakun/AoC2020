use std::env;
use std::io::prelude::*;
use std::io;
use std::fs::File;

#[derive(Debug)]
struct PasswordEntry {
    index1: usize,
    index2: usize,
    c: char,
    password: String
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let c1 = self.password.chars().nth(self.index1 - 1).unwrap();
        let c2 = self.password.chars().nth(self.index2 - 1).unwrap();
        let check1 = c1 == self.c;
        let check2 = c2 == self.c;
        if check1 && check2 {
            false
        } else if check1 || check2 {
            true
        } else {
            false
        }
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
        let index1: usize = token1[0].parse().unwrap();
        let index2: usize = token1[1].parse().unwrap();
        let c: char = tokens[1].trim().chars().next().unwrap(); 
        let password: &str = tokens[2].trim();
        entries.push(PasswordEntry { index1, index2, c, password: password.to_string() });
    }

    Ok(entries)
}
