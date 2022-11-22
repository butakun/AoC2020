use std::io::prelude::*;
use std::env;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn is_valid(&self) -> bool {
        let mut valid: bool;
        valid = Self::digits(&self.byr, 4, 1920, 2002);
        if valid {
            valid = Self::digits(&self.iyr, 4, 2010, 2020);
        }
        if valid {
            valid = Self::digits(&self.eyr, 4, 2020, 2030);
        }
        if valid {
            let len = self.hgt.len();
            if len > 2 {
                let unit = &self.hgt[(len-2)..];
                let v: i32 = match self.hgt[..(len-2)].parse() {
                    Ok(v) => { v },
                    Err(_) => { valid = false; 0 },
                };
                match unit {
                    "cm" => {
                        valid = 150 <= v && v <= 193;
                    },
                    "in" => {
                        valid = 59 <= v && v <= 76;
                    },
                    _ => { valid = false; }
                };
                if unit == "cm" || unit == "in" {
                    let height = &self.hgt[..(len-2)];
                    let height: i32 = match height.parse() {
                        Ok(height) => { height },
                        Err(_) => { valid = false; 0 }
                    };
                } else {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }
        if valid {
            if self.hcl.len() == 7 {
                if self.hcl.starts_with('#') {
                    for c in self.hcl[1..].chars() {
                        let allowed = "0123456789abcdef";
                        valid = allowed.contains(c);
                        if ! valid {
                            break;
                        }
                    }
                } else {
                    valid = false;
                }
            } else {
                valid = false;
            }
        }
        if valid {
            match self.ecl.as_str() {
                "amb" => {},
                "blu" => {},
                "brn" => {},
                "gry" => {},
                "grn" => {},
                "hzl" => {},
                "oth" => {},
                other => { valid = false; }
            }; //blu brn gry grn hzl oth
        }
        if valid {
            if self.pid.len() == 9 {
                for c in self.pid.chars() {
                    let allowed = "0123456789";
                    valid = allowed.contains(c);
                    if ! valid {
                        break;
                    }
                }
            } else {
                valid = false;
            }
        }
        valid
    }

    fn digits(d: &str, count: usize, min: i32, max: i32) -> bool {
        let mut valid: bool;
        if d.len() == count {
            let d: i32 = match d.parse() {
                Ok(d) => {
                    if d >= min && d <= max {
                        valid = true;
                    } else {
                        valid = false;
                    }
                    d
                },
                Err(_) => {
                    valid = false;
                    0
                }
            };
        } else {
            valid = false;
        }
        valid
    }

    fn read_fields(fields: &str) -> HashMap<String, String> {
        let mut dict: HashMap<String, String> = HashMap::new();
        for kv in fields.trim().split(' ') {
            let kv = kv.split_once(':').unwrap();
            dict.insert(kv.0.to_string(), kv.1.to_string());
        }
        dict
    }

    fn from(fields: &str) -> Passport {
       let dict = Self::read_fields(fields);
       let mut passport = Passport{
           byr: match dict.get("byr") { Some(c) => c.to_string(), None => "".to_string() },
           iyr: match dict.get("iyr") { Some(c) => c.to_string(), None => "".to_string() },
           eyr: match dict.get("eyr") { Some(c) => c.to_string(), None => "".to_string() },
           hgt: match dict.get("hgt") { Some(c) => c.to_string(), None => "".to_string() },
           hcl: match dict.get("hcl") { Some(c) => c.to_string(), None => "".to_string() },
           ecl: match dict.get("ecl") { Some(c) => c.to_string(), None => "".to_string() },
           pid: match dict.get("pid") { Some(c) => c.to_string(), None => "".to_string() },
           cid: match dict.get("cid") { Some(c) => c.to_string(), None => "".to_string() },
           };
       passport
    }
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "byr: {}, iyr: {}, eyr: {}, hgt: {}, hcl: {}, ecl: {}, pid: {}, cid: {}",
               self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid, self.cid
               )
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let passports = read(filename).unwrap();

    let mut count_valid = 0;
    for passport in passports {
        let valid = passport.is_valid();
        println!("{} -> {}", passport, valid);
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
            let passport = Passport::from(&fields);
            passports.push(passport);
            fields.clear();
            continue;
        }
        fields.push(' ');
        fields.push_str(&line);
    }
    if fields.trim().len() > 0 {
        let passport = Passport::from(&fields);
        passports.push(passport);
    }
    Ok(passports)
}
