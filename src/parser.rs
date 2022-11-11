use std::fs;

const FILES: [&str; 2] = ["you_died.txt", "you_killed.txt"];

fn read_file(file: &str) -> Vec<String> {
    let contents = fs::read_to_string(file)
        .expect("Couldn't read file");

    contents.lines().map(|s| String::from(s)).collect()
}

pub fn read_files() -> Vec<Vec<String>> {
    let mut files: Vec<Vec<String>> = Vec::new(); 
    for i in 0..FILES.len() {
        let file = FILES[i];
        files.push(read_file(file))
    }
    files
}
