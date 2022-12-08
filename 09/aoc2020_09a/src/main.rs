use std::collections::{VecDeque, HashSet};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let V = read(filename);

    let N = 25;

    let mut vocab: VecDeque<&i64> = V.iter().take(N).collect();

    let target_number;
    for (i, v) in V.iter().skip(N).enumerate() {
        let poss = possible_numbers(&vocab);
        if ! poss.contains(v) {
            target_number = v;
            println!("{v} @ {i} is invalid");
            break;
        }
        vocab.pop_front();
        vocab.push_back(v);
    }

    // part 2
}

fn possible_numbers(vocab: &VecDeque<&i64>) -> HashSet<i64> {
    let vocab_set: HashSet<i64> = HashSet::from_iter(vocab.iter().map(|&&v| v));
    HashSet::from_iter(vocab_set.iter().combinations(2).map(|a| a[0] + a[1]))
}

fn read(filename: &str) -> Vec<i64> {
    std::fs::read_to_string(filename).unwrap().trim().split("\n").map(|n| n.parse().unwrap()).collect()
}
