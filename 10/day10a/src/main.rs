use itertools::Itertools;

fn main() {
    let filename = std::env::args().skip(1).next().unwrap();
    println!("filename = {filename}");

    let mut values = read(&filename);
    for value in values.iter() {
        println!("{value}");
    }

    println!("sorted, inserted zero in front and the device appended");
    values.sort();
    values.insert(0, 0);
    values.push(values.last().unwrap() + 3);
    for value in values.iter() {
        println!("{value}");
    }

    println!("pairwise");
    for (a, b) in values.iter().tuple_windows() {
        println!("{a},{b}");
    }

    let diffs: Vec<u32>  = values.iter()
        .tuple_windows::<(&u32, &u32)>()
        .map(|(a, b)| { b - a })
        .collect();

    println!("diffs");
    for d in diffs.iter() {
        println!("{d}");
    }

    let mut ones = 0u32;
    let mut threes = 0u32;
    diffs.iter()
        .for_each(|v| {
            if *v == 1 {
                ones += 1;
            } else if *v == 3 {
                threes += 1;
            }
        });
    println!("{} x {} = {}", ones, threes, ones * threes);
}

fn read(filename: &str) -> Vec<u32> {
    std::fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split("\n")
        .map(|s| {
            println!("{s}");
            let r = s.parse::<u32>().unwrap();
            r
        })
        .collect()
}
