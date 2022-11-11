use std::fs;

const FILES: [&str; 2] = ["you_died.txt", "you_killed.txt"];

fn read_files(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file)
        .expect("Couldn't read file");

    contents.lines().collect::<Vec<&str>>().into_iter().map(|s| String::from(s)).collect()
}
