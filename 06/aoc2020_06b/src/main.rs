use std::collections::HashSet;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let groups = read(filename);

    let sum: usize = groups.iter().map(|group| do_group(&group)).sum();
    println!("{sum}");
}

fn do_group(group: &Vec<String>) -> usize {
    let mut questions: HashSet<_> = HashSet::from_iter(group[0].chars());

    for person in group.iter().skip(1) {
        let questions2: HashSet<_> = HashSet::from_iter(person.chars());
        questions = questions.intersection(&questions2).map(|c| *c).collect();
    }
    questions.len()
}

fn read(filename: &str) -> Vec<Vec<String>> {
    std::fs::read_to_string(filename).unwrap()
        .split("\n\n")
        .map(
            |group_lines|
                group_lines
                    .trim()
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect()
            )
        .collect()
}
