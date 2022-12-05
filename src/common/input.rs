use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

pub fn read_input(name: &str) -> std::vec::Vec<String> {
    let mut result = Vec::new();
    let path_name = "C:\\Work\\code\\sandbox\\rust\\adventOfCode\\src\\inputs\\".to_owned() + name;
    if let Ok(lines) = read_lines(path_name) {
        for line in lines {
            if let Ok(result_line) = line {
                result.push(result_line);
            }
        }
    }

    return result;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}