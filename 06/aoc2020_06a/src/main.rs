fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];
    let groups = read(filename);

    let sum: usize = groups.iter().map(|group| do_group(&group)).sum();
    println!("{sum}");
}

fn do_group(group: &Vec<String>) -> usize {
    let mut questions = std::collections::HashSet::new();
    for person in group {
        for c in person.chars() {
            questions.insert(c);
        }
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
