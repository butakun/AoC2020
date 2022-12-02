use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seats = read(&args[1]);

    let mut seats: Vec<(u32, u32, u32)> =
    seats.iter()
        .map(|seat| {
            decode(&seat)
        })
        .collect();
    println!("{:?}", seats);

    let mut rows: Vec<u32> = seats.iter().map(|seat| seat.0).collect();
    rows.sort();
    println!("{:?}", rows);

    let first_row = rows[0];
    let last_row = rows[rows.len() - 1];
    println!("{first_row}");
    println!("{last_row}");

    seats.sort_by(|s1, s2| s1.2.cmp(&s2.2));
    println!("{:?}", seats);

    let d: Vec<u32> = 
    seats.iter().zip(seats.iter().skip(1))
        .map(
            |(s1, s2)| {
                let d = s2.2 - s1.2;
                if d > 1 {
                    println!("found a seat between {:?}, {:?}", s1, s2);
                }
                d
            }
            )
        .collect();
}

fn binary_decode(code: &str, f: char, b: char, mut start: u32, mut end: u32) -> u32 {
    for c in code.chars() {
        if c == f {
            end = (end + start) / 2;
        } else if c == b {
            start = (end + start) / 2;
        } else {
            break;
        }
        if (end - start) == 1 {
            break;
        }
    }
    start
}

fn decode(code: &str) -> (u32, u32, u32) {
    let N: u32 = 128;

    let mut start: u32 = 0;
    let mut end = N;
    let row = binary_decode(&code[..7], 'F', 'B', 0, 128);
    let col = binary_decode(&code[7..], 'L', 'R', 0, 8);
    let id = row * 8 + col;
    (row, col, id)
}

fn read(filename: &str) -> Vec<String> {
    let f = File::open(filename).unwrap();
    let reader = BufReader::new(f);
    reader.lines()
        .map(|line| String::from(line.unwrap().trim()))
        .collect()
}
