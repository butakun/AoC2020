use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::fs::File;

#[derive(Debug)]
struct Passport {
    keys: Vec<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let mut valid = true;
        for k in required {
            if ! self.keys.contains(&k.to_string()) {
                valid = false;
                break;
            }
        }
        valid
    }

    fn read_fields(&mut self, fields: &str) {
        for kv in fields.trim().split(' ') {
            let vv = kv.split_once(':').unwrap();
            self.keys.push(vv.0.to_string());
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let passports = read(filename).unwrap();

    let mut count_valid = 0;
    for passport in passports {
        let valid = passport.is_valid();
        println!("{:?} -> {}", passport, valid);
        if valid {
            count_valid += 1;
        }
    }
    println!("{count_valid} passports are valid");
}

fn read(filename: &str) -> Result<Vec<Passport>, std::io::Error> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);

    let mut fields = String::new();

    let mut passports: Vec<Passport> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.trim().len() == 0 {
            let mut passport = Passport{keys: Vec::new()};
            passport.read_fields(&fields);
            passports.push(passport);
            fields.clear();
            continue;
        }
        fields.push(' ');
        fields.push_str(&line);
    }
    if fields.trim().len() > 0 {
        let mut passport = Passport{keys: Vec::new()};
        passport.read_fields(&fields);
        passports.push(passport);
    }
    Ok(passports)
}
