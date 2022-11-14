use std::env;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{}, {}", args[0], args[1]);

    let filename = &args[1];
    let values = read(filename)?;
    let N = values.len();

    for i in 0..N {
        for j in (i+1)..N {
            for k in (j+1)..N {
                let v1 = values[i];
                let v2 = values[j];
                let v3 = values[k];
                let sum = v1 + v2 + v3;
                if sum == 2020 {
                    let product = v1 * v2 * v3;
                    println!("{v1}, {v2}, {v3}, {sum}, {product}");
                    break;
                }
            }
        }
    }

    Ok(())
}

fn read(filename: &str) -> Result<Vec<i32>, io::Error> {
    let f = File::open(filename).expect("failed opening {filename}");
    let mut reader = BufReader::new(f);
    let mut values: Vec<i32> = Vec::new();

    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        if n == 0 {
            break;
        }
        let value: i32 = match line.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("error parsing {line}");
                continue;
            }
        };
        values.push(value);
    }
    
    Ok(values)
}
